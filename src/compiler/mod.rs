mod frame;
mod value;
mod var;

use crate::compiler::frame::Frame;
use crate::compiler::value::Value;
use crate::compiler::var::Var;
use crate::expression;
use crate::llvm;
use crate::llvm::BasicBlock;
use crate::llvm::PassManager;
use crate::parser;
use crate::parser::Program;
use crate::visitor::Visitor;
use std::convert::TryInto;
use std::error::Error;

const MAIN_FUNCTION: &str = "__main__";

pub trait Compile {
    fn compile(&mut self) -> Result<(), Box<dyn Error>>;
}

pub struct Compiler {
    program: Program,
    engine: llvm::Engine,
    context: llvm::Context,
    module: llvm::Module,
    builder: llvm::Builder,
    fpm: PassManager,
    opt: bool,
    stack: Vec<Frame>,
    after_loop_blocks: Vec<BasicBlock>,
}

impl Visitor<Value> for Compiler {
    fn visit_binary(&mut self, expr: &expression::Binary) -> Value {
        match expr.operator {
            expression::Operator::Plus => {
                let l = match self.walk(&expr.left) {
                    Value::Numeric(p) => p,
                    _ => panic!("panic"),
                };

                let r = match self.walk(&expr.right) {
                    Value::Numeric(p) => p,
                    _ => panic!("panic"),
                };

                Value::Numeric(self.builder.build_fadd(l, r, ""))
            }
            expression::Operator::Asterisk => {
                let l = match self.walk(&expr.left) {
                    Value::Numeric(p) => p,
                    _ => panic!("panic"),
                };

                let r = match self.walk(&expr.right) {
                    Value::Numeric(p) => p,
                    _ => panic!("panic"),
                };

                Value::Numeric(self.builder.build_fmul(l, r, ""))
            }
            expression::Operator::LessOrEqual => {
                let l = match self.walk(&expr.left) {
                    Value::Numeric(p) => p,
                    _ => panic!("panic"),
                };

                let r = match self.walk(&expr.right) {
                    Value::Numeric(p) => p,
                    _ => panic!("panic"),
                };

                Value::Bool(self.builder.build_fcmp(l, r, llvm::Cmp::LessOrEqual, ""))
            }
            expression::Operator::Less => {
                let l = match self.walk(&expr.left) {
                    Value::Numeric(p) => p,
                    _ => panic!("panic"),
                };

                let r = match self.walk(&expr.right) {
                    Value::Numeric(p) => p,
                    _ => panic!("panic"),
                };

                Value::Bool(self.builder.build_fcmp(l, r, llvm::Cmp::Less, ""))
            }
            expression::Operator::Greater => {
                let l = match self.walk(&expr.left) {
                    Value::Numeric(p) => p,
                    _ => panic!("panic"),
                };

                let r = match self.walk(&expr.right) {
                    Value::Numeric(p) => p,
                    _ => panic!("panic"),
                };

                Value::Bool(self.builder.build_fcmp(l, r, llvm::Cmp::Greater, ""))
            }
            expression::Operator::Equal => {
                let l = match self.walk(&expr.left) {
                    Value::Numeric(p) => p,
                    _ => panic!("panic"),
                };

                let r = match self.walk(&expr.right) {
                    Value::Numeric(p) => p,
                    _ => panic!("panic"),
                };

                Value::Bool(self.builder.build_fcmp(l, r, llvm::Cmp::Equal, ""))
            }
            expression::Operator::Minus => {
                let l = match self.walk(&expr.left) {
                    Value::Numeric(p) => p,
                    _ => panic!("panic"),
                };

                let r = match self.walk(&expr.right) {
                    Value::Numeric(p) => p,
                    _ => panic!("panic"),
                };

                Value::Numeric(self.builder.build_fsub(l, r, ""))
            }
            expression::Operator::Slash => {
                let l = match self.walk(&expr.left) {
                    Value::Numeric(p) => p,
                    _ => panic!("panic"),
                };

                let r = match self.walk(&expr.right) {
                    Value::Numeric(p) => p,
                    _ => panic!("panic"),
                };

                Value::Numeric(self.builder.build_fdiv(l, r, ""))
            }
            expression::Operator::GreaterOrEqual => {
                let l = match self.walk(&expr.left) {
                    Value::Numeric(p) => p,
                    _ => panic!("panic"),
                };

                let r = match self.walk(&expr.right) {
                    Value::Numeric(p) => p,
                    _ => panic!("panic"),
                };

                Value::Bool(self.builder.build_fcmp(l, r, llvm::Cmp::GreaterOrEqual, ""))
            }
            expression::Operator::NotEqual => {
                let l = match self.walk(&expr.left) {
                    Value::Numeric(p) => p,
                    _ => panic!("panic"),
                };

                let r = match self.walk(&expr.right) {
                    Value::Numeric(p) => p,
                    _ => panic!("panic"),
                };

                Value::Bool(self.builder.build_fcmp(l, r, llvm::Cmp::NotEqual, ""))
            }
            _ => todo!("{:?}", expr.operator),
        }
    }

    fn visit_numeric(&mut self, f: &f64) -> Value {
        Value::Numeric(self.context.const_double(*f))
    }

    fn visit_conditional(&mut self, expr: &expression::Conditional) -> Value {
        let fun = self.builder.get_insert_block().get_parent();

        let then_block = self.context.append_basic_block(&fun, "then");
        let else_block = self.context.append_basic_block(&fun, "else");
        let after_if_block = self.context.append_basic_block(&fun, "afterif");

        let predicate = self.walk(&expr.predicate);
        match predicate {
            Value::Bool(b) => {
                self.builder.build_cond_br(&b, &then_block, &else_block);
                self.builder.position_builder_at_end(&then_block);

                let mut is_break = false;
                for stmt in &expr.body {
                    self.walk(stmt);

                    if matches!(stmt, expression::Expression::Break) {
                        is_break = true;
                        break;
                    }
                }

                if !is_break {
                    self.builder.create_br(&after_if_block);
                }
                self.builder.position_builder_at_end(&else_block);
                for stmt in &expr.else_body {
                    self.walk(stmt);
                }
                self.builder.create_br(&after_if_block);

                self.builder.position_builder_at_end(&after_if_block);

                Value::Null
            }
            _ => todo!("{:?}", predicate),
        }
    }

    fn visit_assignment(&mut self, expr: &expression::Assignment) -> Value {
        let literal = match &*expr.left {
            expression::Expression::Identifier(literal) => literal,
            _ => panic!("panic"),
        };

        let val = self.walk(&expr.right);
        self.set_var(literal, val);

        if let expression::Expression::FuncDecl(e) = &*expr.right {
            self.build_function(val, &*e)
        }

        val
    }

    fn visit_unary(&mut self, expr: &expression::Unary) -> Value {
        match expr.operator {
            expression::Operator::Minus => {
                let r = match self.walk(&expr.right) {
                    Value::Numeric(p) => p,
                    _ => panic!("panic"),
                };

                Value::Numeric(self.builder.build_fneg(r, ""))
            }
            _ => panic!(
                "operator {:?} is not valid for unary operations",
                expr.operator
            ),
        }
    }

    fn visit_grouping(&mut self, expr: &expression::Expression) -> Value {
        self.walk(expr)
    }

    fn visit_func_call(&mut self, expr: &expression::FuncCall) -> Value {
        match &*expr.calee {
            expression::Expression::Identifier(literal) => match literal.as_str() {
                "print" => {
                    if expr.args.len() != 1 {
                        panic!("arity 1 expected");
                    }
                    let res = self.walk(&expr.args[0]);

                    match res {
                        Value::GlobalString(s) => {
                            let void_type = self.context.void_type();
                            let i8_pointer_type = self.context.i8_type().pointer_type(0);
                            let func_type =
                                self.context
                                    .function_type(void_type, &[i8_pointer_type], false);
                            let printf_func = self
                                .module
                                .get_function("printf")
                                .unwrap_or_else(|| self.module.add_function("printf", func_type));
                            let p = self.builder.build_bitcast(&s, i8_pointer_type, "");
                            self.builder.build_call(&printf_func, &[p], "");
                        }
                        Value::String(s) => {
                            let void_type = self.context.void_type();
                            let i8_pointer_type = self.context.i8_type().pointer_type(0);
                            let func_type =
                                self.context
                                    .function_type(void_type, &[i8_pointer_type], false);
                            let printf_func = self
                                .module
                                .get_function("printf")
                                .unwrap_or_else(|| self.module.add_function("printf", func_type));
                            let p = self.builder.build_bitcast(&s, i8_pointer_type, "");
                            self.builder.build_call(&printf_func, &[p], "");
                            self.builder.build_free(s);
                        }
                        _ => panic!("type error, not a string"),
                    }

                    Value::Null
                }
                "string" => {
                    if expr.args.len() != 1 {
                        panic!("arity 1 expected");
                    }
                    let res = self.walk(&expr.args[0]);

                    match res {
                        Value::Numeric(f) => {
                            let i8_pointer_type = self.context.i8_type().pointer_type(0);

                            let func_type = self.context.function_type(
                                i8_pointer_type,
                                &[i8_pointer_type, i8_pointer_type, self.context.double_type()],
                                true,
                            );
                            let sprintf = self
                                .module
                                .get_function("sprintf")
                                .unwrap_or_else(|| self.module.add_function("sprintf", func_type));

                            let arr_type = self.context.array_type(self.context.i8_type(), 100);

                            let format_str = self.builder.build_global_string_ptr("%f", "");

                            let arr = self.builder.build_malloc(arr_type, "");
                            let p = self.builder.build_bitcast(&arr, i8_pointer_type, "");
                            self.builder.build_call(&sprintf, &[p, format_str, f], "");

                            Value::String(arr)
                        }
                        _ => panic!("type error, not a string"),
                    }
                }
                "len" => {
                    let args: Vec<llvm::Value> = expr
                        .args
                        .iter()
                        .map(|arg| match self.walk(arg) {
                            Value::Vec(n) => n,
                            Value::Numeric(n) => n,
                            _ => panic!("{:?}", self.walk(arg)),
                        })
                        .collect();

                    let fun_type = self.context.function_type(
                        self.context.double_type(),
                        &[self.context.double_type().pointer_type(0)],
                        false,
                    );

                    let fun_addr = stdlib::len as usize;
                    let ptr = self.context.const_u64_to_ptr(
                        self.context.const_u64(fun_addr.try_into().unwrap()),
                        fun_type.pointer_type(0),
                    );

                    Value::Numeric(self.builder.build_call(&ptr, &args, ""))
                }
                "vec_new" => {
                    let fun_type = self.context.function_type(
                        self.context.double_type().pointer_type(0),
                        &[],
                        false,
                    );

                    let fun_addr = stdlib::vec_new as usize;
                    let ptr = self.context.const_u64_to_ptr(
                        self.context.const_u64(fun_addr.try_into().unwrap()),
                        fun_type.pointer_type(0),
                    );
                    Value::Vec(self.builder.build_call(&ptr, &[], ""))
                }
                "vec_mut" => {
                    let args: Vec<llvm::Value> = expr
                        .args
                        .iter()
                        .map(|arg| match self.walk(arg) {
                            Value::Vec(n) => n,
                            Value::Numeric(n) => n,
                            _ => todo!("{:?}", self.walk(arg)),
                        })
                        .collect();

                    let fun_type = self.context.function_type(
                        self.context.void_type(),
                        &[
                            self.context.double_type().pointer_type(0),
                            self.context.double_type(),
                            self.context.double_type(),
                        ],
                        false,
                    );

                    let fun_addr = stdlib::vec_mut as usize;
                    let ptr = self.context.const_u64_to_ptr(
                        self.context.const_u64(fun_addr.try_into().unwrap()),
                        fun_type.pointer_type(0),
                    );

                    Value::Vec(self.builder.build_call(&ptr, &args, ""))
                }
                "sqrt" => {
                    let args: Vec<llvm::Value> = expr
                        .args
                        .iter()
                        .map(|arg| match self.walk(arg) {
                            Value::Numeric(n) => n,
                            _ => panic!("{:?}", self.walk(arg)),
                        })
                        .collect();

                    let fun_type = self.context.function_type(
                        self.context.double_type(),
                        &[self.context.double_type()],
                        false,
                    );

                    let fun_addr = stdlib::sqrt as usize;
                    let ptr = self.context.const_u64_to_ptr(
                        self.context.const_u64(fun_addr.try_into().unwrap()),
                        fun_type.pointer_type(0),
                    );

                    Value::Numeric(self.builder.build_call(&ptr, &args, ""))
                }
                "vec_get" => {
                    let args: Vec<llvm::Value> = expr
                        .args
                        .iter()
                        .map(|arg| match self.walk(arg) {
                            Value::Vec(n) => n,
                            Value::Numeric(n) => n,
                            _ => panic!("{:?}", self.walk(arg)),
                        })
                        .collect();

                    let fun_type = self.context.function_type(
                        self.context.double_type(),
                        &[
                            self.context.double_type().pointer_type(0),
                            self.context.double_type(),
                        ],
                        false,
                    );

                    let fun_addr = stdlib::vec_get as usize;
                    let ptr = self.context.const_u64_to_ptr(
                        self.context.const_u64(fun_addr.try_into().unwrap()),
                        fun_type.pointer_type(0),
                    );

                    Value::Numeric(self.builder.build_call(&ptr, &args, ""))
                }
                _ => match &self.get_var(literal) {
                    Some(Value::Function {
                        typ: _,
                        val,
                        return_type,
                    }) => {
                        let args: Vec<llvm::Value> = expr
                            .args
                            .iter()
                            .map(|arg| match self.walk(arg) {
                                Value::Numeric(n) => n,
                                Value::Ptr(n) => n,
                                Value::Vec(v) => {
                                    let fun_type = self.context.function_type(
                                        self.context.void_type(),
                                        &[self.context.double_type().pointer_type(0)],
                                        false,
                                    );
                                    let fun_addr = stdlib::vec_reference as usize;
                                    let ptr = self.context.const_u64_to_ptr(
                                        self.context.const_u64(fun_addr.try_into().unwrap()),
                                        fun_type.pointer_type(0),
                                    );
                                    self.builder.build_call(&ptr, &[v], "");
                                    v
                                }
                                Value::Function { val, .. } => val,
                                Value::GlobalString(s) => s,
                                _ => todo!("{:?}", self.walk(arg)),
                            })
                            .collect();

                        match return_type {
                            parser::Type::Vector => {
                                Value::Vec(self.builder.build_call(val, &args, ""))
                            }
                            parser::Type::Numeric => {
                                Value::Numeric(self.builder.build_call(val, &args, ""))
                            }
                            parser::Type::String => {
                                Value::String(self.builder.build_call(val, &args, ""))
                            }
                            parser::Type::Function => Value::Function {
                                val: self.builder.build_call(val, &args, ""),
                                typ: self
                                    .context
                                    .function_type(self.context.void_type(), &[], false)
                                    .pointer_type(0),
                                return_type: parser::Type::Null,
                            },
                            parser::Type::Null => {
                                self.builder.build_call(val, &args, "");
                                Value::Null
                            }
                            parser::Type::Ptr => {
                                Value::Ptr(self.builder.build_call(val, &args, ""))
                            }
                        }
                    }
                    Some(e) => panic!("unexpected {:?}", e),
                    None => panic!("undefined function {}", literal),
                },
            },
            _ => panic!("evaluation error"),
        }
    }

    fn visit_while(&mut self, expr: &expression::While) -> Value {
        let predicate = self.walk(&expr.predicate);
        match predicate {
            Value::Bool(b) => {
                let fun = self.builder.get_insert_block().get_parent();

                let loop_block = self.context.append_basic_block(&fun, "loop");
                let after_loop_block = self.context.append_basic_block(&fun, "afterloop");

                self.builder
                    .build_cond_br(&b, &loop_block, &after_loop_block);

                self.builder.position_builder_at_end(&loop_block);

                self.after_loop_blocks.push(after_loop_block);
                let mut is_break = false;

                for stmt in &expr.body {
                    self.walk(stmt);
                    if matches!(stmt, expression::Expression::Break) {
                        is_break = true;
                        break;
                    }
                }

                if !is_break {
                    let term_pred = self.walk(&expr.predicate);

                    match term_pred {
                        Value::Bool(b) => {
                            self.builder
                                .build_cond_br(&b, &loop_block, &after_loop_block);
                        }
                        _ => panic!("type error"),
                    }
                    self.builder.position_builder_at_end(&after_loop_block);
                }

                self.after_loop_blocks.pop();
            }
            _ => panic!("type error"),
        }

        Value::Null
    }

    fn visit_identifier(&mut self, expr: &str) -> Value {
        self.get_var(expr)
            .unwrap_or_else(|| panic!("undefined variable {}", expr))
        // match &self.get_var(expr) {
        //     Some(Value::Numeric(n)) => Value::Numeric(self.builder.build_load(n, expr)),
        //     Some(Value::Function {
        //         typ,
        //         val,
        //         return_type,
        //     }) => Value::Function {
        //         typ: *typ,
        //         val: *val,
        //         return_type: return_type.clone(),
        //     },
        //     Some(Value::Vec(n)) => Value::Vec(*n),
        //     Some(Value::Pending) | None => panic!("undefined identifier {}", expr),
        //     _ => todo!("{:?}", &self.get_var(expr)),
        // }
    }

    fn visit_string(&mut self, expr: &str) -> Value {
        let with_newlines = expr.to_string().replace("\\n", "\n");
        Value::GlobalString(
            self.builder
                .build_global_string_ptr(with_newlines.as_str(), ""),
        )
    }

    fn visit_bool(&mut self, expr: &bool) -> Value {
        Value::Bool(self.context.const_bool(*expr))
    }

    fn visit_break(&mut self) -> Value {
        let after_loop_block = self.after_loop_blocks.first().unwrap();

        self.builder.build_br(after_loop_block);
        self.builder.position_builder_at_end(after_loop_block);

        Value::Break
    }

    fn visit_program(&mut self, program: parser::Program) -> Value {
        let void_t = self.context.void_type();
        let sum_type = self.context.function_type(void_t, &[], false);
        let sum_fun = self.module.add_function(MAIN_FUNCTION, sum_type);
        self.stack.push(Frame::new(sum_fun));
        let block = self.context.append_basic_block(&sum_fun, "entry");
        self.builder.position_builder_at_end(&block);

        for stmt in program.body {
            self.walk(&stmt);
        }

        self.builder.build_ret_void();

        sum_fun.verify_function().unwrap_or_else(|_x| {
            println!("IR Dump:");
            self.dump_ir();
            panic!()
        });

        if self.opt {
            self.fpm.run(&sum_fun);
        }

        self.stack.pop();

        Value::Null
    }

    fn visit_func_decl(&mut self, expr: &expression::FuncDecl) -> Value {
        let types: Vec<llvm::Type> = expr
            .params
            .iter()
            .map(|arg| self.get_llvm_type(arg.typ))
            .collect();

        let fun_type =
            self.context
                .function_type(self.get_llvm_type(expr.return_type), &types, false);
        let fun = self.module.add_function("fun", fun_type);

        Value::Function {
            return_type: expr.return_type,
            typ: fun_type,
            val: fun,
        }
    }

    fn visit_load(&mut self, name: &str) -> Value {
        self.context.load_libary_permanently(name);
        Value::Null
    }

    fn visit_extern(&mut self, extern_stmt: &expression::Extern) -> Value {
        let types: Vec<llvm::Type> = extern_stmt
            .types
            .iter()
            .map(|typ| self.get_llvm_type(*typ))
            .collect();

        let fun_type =
            self.context
                .function_type(self.get_llvm_type(extern_stmt.return_type), &types, false);
        let fun = self
            .module
            .add_function(extern_stmt.name.as_str(), fun_type);

        Value::Function {
            val: fun,
            typ: fun_type,
            return_type: extern_stmt.return_type,
        }
    }
}

impl Compile for Compiler {
    fn compile(&mut self) -> Result<(), Box<dyn Error>> {
        self.visit_program(self.program.clone());
        Ok(())
    }
}

impl Compiler {
    pub fn dump_ir(&self) {
        println!("{}", self.module);
    }

    pub fn ir_string(&self) -> String {
        format!("{}", self.module)
    }

    fn set_var(&mut self, literal: &str, val: Value) {
        let typ = match val {
            Value::Ptr(_) => self.context.void_type().pointer_type(0),
            Value::Numeric(_) => self.context.double_type(),
            Value::Vec(_) => self.context.double_type().pointer_type(0),
            Value::Null => self.context.void_type(),
            Value::String(_) => self.context.i8_type().pointer_type(0),
            Value::GlobalString(_) => self.context.i8_type().pointer_type(0),
            Value::Bool(_) => self.context.i1_type(),
            Value::Function { typ, .. } => typ.pointer_type(0),
            Value::Break => self.context.void_type(),
        };

        let existing_ptr: Option<llvm::Value> = match self.get_var_ptr(literal) {
            Some(v) => match v {
                Var::Numeric(v)
                | Var::String(v)
                | Var::GlobalString(v)
                | Var::Vec(v)
                | Var::Bool(v)
                | Var::Function { val: v, .. } => Some(v),
                _ => todo!(),
            },
            None => None,
        };
        if self.get_var(literal).is_none() && self.stack.len() <= 1 {
            let ptr = match val {
                Value::Null => unreachable!(),
                Value::Numeric(_)
                | Value::Ptr(_)
                | Value::String(_)
                | Value::GlobalString(_)
                | Value::Break
                | Value::Vec(_)
                | Value::Bool(_) => self.module.add_global(typ, literal),
                Value::Function { val: v, .. } => v,
            };

            match val {
                Value::Numeric(_) | Value::GlobalString(_) | Value::Vec(_) => {
                    ptr.set_initializer(self.context.const_double(0.0));
                }
                Value::Ptr(_) => {
                    ptr.set_initializer(self.context.const_double(0.0));
                }
                Value::Null => unreachable!(),
                Value::String(_) => todo!(),
                Value::Function { .. } => (),
                Value::Break => unreachable!(),
                Value::Bool(_) => {
                    ptr.set_initializer(self.context.const_bool(false));
                }
            }

            let var = match val {
                Value::Numeric(_) => Var::Numeric(ptr),
                Value::Ptr(_) => Var::Ptr(ptr),
                Value::Null => Var::Null,
                Value::String(_) => Var::String(ptr),
                Value::GlobalString(_) => Var::GlobalString(ptr),
                Value::Vec(_) => Var::Vec(ptr),
                Value::Bool(_) => Var::Bool(ptr),
                Value::Break => Var::Null,
                Value::Function {
                    typ,
                    return_type,
                    val,
                    ..
                } => Var::Function {
                    val,
                    typ,
                    return_type,
                },
            };

            self.stack
                .last_mut()
                .unwrap()
                .set(&self.context, &self.builder, literal, var);

            match val {
                Value::Numeric(v)
                | Value::Ptr(v)
                | Value::String(v)
                | Value::GlobalString(v)
                | Value::Vec(v)
                | Value::Bool(v) => self.builder.create_store(v, &ptr),
                Value::Function { val: v, .. } => v,
                _ => todo!("{:?}", val),
            };
        } else {
            let ptr = existing_ptr.unwrap_or_else(|| self.builder.build_alloca(typ, literal));

            let var = match val {
                Value::Numeric(_) => Var::Numeric(ptr),
                Value::Ptr(_) => Var::Ptr(ptr),
                Value::Null => Var::Null,
                Value::String(_) => Var::String(ptr),
                Value::GlobalString(_) => Var::GlobalString(ptr),
                Value::Vec(_) => Var::Vec(ptr),
                Value::Bool(_) => Var::Bool(ptr),
                Value::Break => Var::Null,
                Value::Function {
                    typ,
                    return_type,
                    val,
                    ..
                } => Var::Function {
                    val,
                    typ,
                    return_type,
                },
            };

            self.stack
                .last_mut()
                .unwrap()
                .set(&self.context, &self.builder, literal, var);

            match val {
                Value::Numeric(v)
                | Value::String(v)
                | Value::GlobalString(v)
                | Value::Vec(v)
                | Value::Bool(v) => self.builder.create_store(v, &ptr),
                Value::Function { val: v, .. } => v,
                _ => todo!("{:?}", val),
            };
        }
    }

    fn get_var_ptr(&mut self, literal: &str) -> Option<Var> {
        for frame in self.stack.iter().rev() {
            if let Some(v) = frame.get(literal) {
                return Some(*v);
            };
        }
        None
    }

    fn get_var(&mut self, literal: &str) -> Option<Value> {
        match self.get_var_ptr(literal) {
            Some(v) => {
                let val: llvm::Value = match v {
                    Var::Numeric(p)
                    | Var::String(p)
                    | Var::GlobalString(p)
                    | Var::Vec(p)
                    | Var::Ptr(p)
                    | Var::Bool(p) => self.builder.build_load(&p, ""),
                    Var::Function { val: p, .. } => p,
                    _ => todo!(),
                };

                Some(match v {
                    Var::Numeric(_) => Value::Numeric(val),
                    Var::Ptr(_) => Value::Ptr(val),
                    Var::String(_) => Value::String(val),
                    Var::GlobalString(_) => Value::GlobalString(val),
                    Var::Vec(_) => Value::Vec(val),
                    Var::Bool(_) => Value::Bool(val),
                    Var::Function {
                        val,
                        return_type,
                        typ,
                    } => Value::Function {
                        val,
                        return_type,
                        typ,
                    },
                    _ => todo!(),
                })
            }
            None => None,
        }
    }

    pub fn run(&self) {
        self.engine.call(MAIN_FUNCTION);
    }

    pub fn no_opt(&mut self) {
        self.opt = false;
    }

    fn build_function(&mut self, fun_compiler_val: Value, expr: &expression::FuncDecl) {
        let fun = match fun_compiler_val {
            Value::Function { val, .. } => val,
            _ => panic!(),
        };

        let curr = self.builder.get_insert_block();

        let block = self.context.append_basic_block(&fun, "entry");
        self.builder.position_builder_at_end(&block);

        self.stack.push(Frame::new(fun));

        for (i, param) in expr.params.iter().enumerate() {
            self.set_var(
                param.name.as_str(),
                match param.typ {
                    parser::Type::Vector => Value::Vec(fun.get_param(i.try_into().unwrap())),
                    parser::Type::Numeric => Value::Numeric(fun.get_param(i.try_into().unwrap())),
                    parser::Type::Ptr => Value::Ptr(fun.get_param(i.try_into().unwrap())),
                    parser::Type::String => Value::String(fun.get_param(i.try_into().unwrap())),
                    parser::Type::Null => Value::Null,
                    parser::Type::Function => Value::Function {
                        val: fun.get_param(i.try_into().unwrap()),
                        typ: self
                            .context
                            .function_type(self.context.void_type(), &[], false)
                            .pointer_type(0),
                        return_type: parser::Type::Null,
                    },
                },
            )
        }

        let mut last_val = Value::Null;
        for stmt in expr.body.clone() {
            last_val = self.walk(&stmt);
        }

        let frame = self.stack.pop().unwrap();

        let ret_val = match last_val {
            Value::Null => None,
            Value::Numeric(n) => Some(n),
            Value::Vec(n) => {
                let fun_type = self.context.function_type(
                    self.context.void_type(),
                    &[self.context.double_type().pointer_type(0)],
                    false,
                );
                let fun_addr = stdlib::vec_reference as usize;
                let ptr = self.context.const_u64_to_ptr(
                    self.context.const_u64(fun_addr.try_into().unwrap()),
                    fun_type.pointer_type(0),
                );
                self.builder.build_call(&ptr, &[n], "");

                Some(n)
            }
            _ => todo!("{:?}", last_val),
        };

        frame.release(&self.context, &self.builder);

        match ret_val {
            Some(v) => self.builder.build_ret(v),
            None => self.builder.build_ret_void(),
        };

        self.builder.position_builder_at_end(&curr);

        fun.verify_function().unwrap_or_else(|_x| {
            println!("IR Dump:");
            self.dump_ir();
            panic!("Function verification failed")
        });

        if self.opt {
            self.fpm.run(&fun);
        }
    }

    fn get_llvm_type(&self, typ: parser::Type) -> llvm::Type {
        match typ {
            parser::Type::Vector => self.context.double_type().pointer_type(0),
            parser::Type::Numeric => self.context.double_type(),
            parser::Type::Function => self
                .context
                .function_type(self.context.void_type(), &[], false)
                .pointer_type(0),
            parser::Type::Null => self.context.void_type(),
            parser::Type::Ptr => self.context.void_type().pointer_type(0),
            parser::Type::String => self.context.i8_type().pointer_type(0),
        }
    }

    pub fn new(program: Program) -> Self {
        let context = llvm::Context::new();
        let module = llvm::Module::new("main", &context);
        let builder = llvm::Builder::new(&context);
        let engine = llvm::Engine::new(&module);

        let fpm = llvm::PassManager::new(&module);

        Compiler {
            program,
            context,
            module,
            builder,
            engine,
            stack: vec![],
            after_loop_blocks: vec![],
            fpm,
            opt: true,
        }
    }
}
