mod frame;
mod value;
mod var;

use crate::compiler::frame::Frame;
use crate::compiler::value::Value;
use crate::compiler::var::Var;
use crate::expression;
use crate::llvm;
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
                for stmt in &expr.body {
                    self.walk(stmt);
                }
                self.builder.create_br(&after_if_block);

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
        val
    }

    fn visit_unary(&mut self, _: &expression::Unary) -> Value {
        todo!()
    }

    fn visit_grouping(&mut self, _: &expression::Expression) -> Value {
        todo!()
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

                            // 							/// CreateEntryBlockAlloca - Create an alloca instruction in the entry block of
                            // /// the function.  This is used for mutable variables etc.
                            // static AllocaInst *CreateEntryBlockAlloca(Function *TheFunction,
                            //                                           const std::string &VarName) {
                            //   IRBuilder<> TmpB(&TheFunction->getEntryBlock(),
                            //                  TheFunction->getEntryBlock().begin());
                            //   return TmpB.CreateAlloca(Type::getDoubleTy(TheContext), 0,
                            //                            VarName.c_str());
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
                            Value::Vec(n) => self.builder.build_load(&n, ""),
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
                "vecnew" => {
                    let fun_type = self.context.function_type(
                        self.context.double_type().pointer_type(0),
                        &[],
                        false,
                    );

                    let fun_addr = stdlib::vecnew as usize;
                    let ptr = self.context.const_u64_to_ptr(
                        self.context.const_u64(fun_addr.try_into().unwrap()),
                        fun_type.pointer_type(0),
                    );
                    Value::Vec(self.builder.build_call(&ptr, &[], ""))
                }
                "vecset" => {
                    let args: Vec<llvm::Value> = expr
                        .args
                        .iter()
                        .map(|arg| match self.walk(arg) {
                            Value::Vec(n) => self.builder.build_load(&n, ""),
                            Value::Numeric(n) => n,
                            _ => todo!("{:?}", self.walk(arg)),
                        })
                        .collect();

                    let fun_type = self.context.function_type(
                        self.context.double_type().pointer_type(0),
                        &[
                            self.context.double_type().pointer_type(0),
                            self.context.double_type(),
                            self.context.double_type(),
                        ],
                        false,
                    );

                    let fun_addr = stdlib::vecset as usize;
                    let ptr = self.context.const_u64_to_ptr(
                        self.context.const_u64(fun_addr.try_into().unwrap()),
                        fun_type.pointer_type(0),
                    );

                    Value::Vec(self.builder.build_call(&ptr, &args, "vecset"))
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
                "vecget" => {
                    let args: Vec<llvm::Value> = expr
                        .args
                        .iter()
                        .map(|arg| match self.walk(arg) {
                            Value::Vec(n) => self.builder.build_load(&n, ""),
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

                    let fun_addr = stdlib::vecget as usize;
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
                                Value::Vec(v) => v,
                                Value::Function { val, .. } => val,
                                _ => todo!("{:?}", self.walk(arg)),
                            })
                            .collect();

                        match return_type {
                            parser::Type::Vector => Value::Vec(
                                self.builder
                                    .build_load(&self.builder.build_call(val, &args, ""), ""),
                            ),
                            parser::Type::Numeric => {
                                Value::Numeric(self.builder.build_call(val, &args, ""))
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
                        }
                    }
                    Some(Value::Pending) => {
                        let args: Vec<llvm::Value> = expr
                            .args
                            .iter()
                            .map(|arg| match self.walk(arg) {
                                Value::Numeric(n) => n,
                                _ => todo!("{:?}", self.walk(arg)),
                            })
                            .collect();

                        Value::Numeric(self.builder.build_call(
                            &self.stack.last().unwrap().fun,
                            &args,
                            literal,
                        ))
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

                for stmt in &expr.body {
                    self.walk(stmt);
                }
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
        todo!()
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
            .map(|arg| match arg.typ {
                parser::Type::Vector => self.context.double_type().pointer_type(0),
                parser::Type::Numeric => self.context.double_type(),
                parser::Type::Function => self
                    .context
                    .function_type(self.context.void_type(), &[], false)
                    .pointer_type(0),
                parser::Type::Null => self.context.void_type(),
            })
            .collect();

        let return_type = match expr.return_type {
            parser::Type::Vector => self.context.double_type().pointer_type(0),
            parser::Type::Numeric => self.context.double_type(),
            parser::Type::Function => self
                .context
                .function_type(self.context.void_type(), &[], false)
                .pointer_type(0),
            parser::Type::Null => self.context.void_type(),
        };

        let fun_type = self.context.function_type(return_type, &types, false);

        let curr = self.builder.get_insert_block();

        let fun = self.module.add_function("fun", fun_type);
        let block = self.context.append_basic_block(&fun, "entry");
        self.builder.position_builder_at_end(&block);

        self.stack.push(Frame::new(fun));

        for (i, param) in expr.params.iter().enumerate() {
            self.set_var(
                param.name.as_str(),
                match param.typ {
                    parser::Type::Vector => Value::Vec(fun.get_param(i.try_into().unwrap())),
                    parser::Type::Numeric => Value::Numeric(fun.get_param(i.try_into().unwrap())),
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

        // for (_, val) in &self.stack.last().unwrap().env {
        //     match val {
        //         Value::Vec(v) => {
        //             let fun_type = self.context.function_type(
        //                 self.context.void_type(),
        //                 &[self.context.void_type().pointer_type(0)],
        //                 false,
        //             );

        //             let fun_addr = unsafe {
        //                 std::mem::transmute::<*const (), u64>(stdlib::vecfree as *const ())
        //             };
        //             let ptr = self.context.const_u64_to_ptr(
        //                 self.context.const_u64(fun_addr),
        //                 fun_type.pointer_type(0),
        //             );
        //             self.builder
        //                 .build_call(&ptr, &[self.builder.build_load(v, "")], "");
        //         }
        //         _ => (),
        //     }
        // }

        self.stack.pop();

        match last_val {
            Value::Null => self.builder.build_ret_void(),
            Value::Numeric(n) => self.builder.build_ret(n),
            Value::Vec(n) => self.builder.build_ret(n),
            _ => todo!("{:?}", last_val),
        };

        self.builder.position_builder_at_end(&curr);

        fun.verify_function().unwrap_or_else(|_x| {
            println!("IR Dump:");
            self.dump_ir();
            panic!()
        });

        if self.opt {
            self.fpm.run(&fun);
        }

        Value::Function {
            return_type: expr.return_type.clone(),
            typ: fun_type,
            val: fun,
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
            Value::Numeric(_) => self.context.double_type(),
            Value::Pending => self.context.void_type(),
            Value::Vec(_) => self.context.double_type().pointer_type(0),
            Value::Null => self.context.void_type(),
            Value::String(_) => self.context.i8_type().pointer_type(0),
            Value::GlobalString(_) => self.context.i8_type().pointer_type(0),
            Value::Bool(_) => self.context.i1_type(),
            Value::Function { typ, .. } => typ.pointer_type(0),
        };

        let existing_ptr: Option<llvm::Value> = match self.get_var_ptr(literal) {
            Some(v) => match v {
                Var::Numeric(v)
                | Var::String(v)
                | Var::GlobalString(v)
                | Var::Vec(v)
                | Var::Bool(v)
                | Var::Function(v) => Some(v),
                _ => todo!(),
            },
            None => None,
        };

        let ptr = existing_ptr.unwrap_or_else(|| self.builder.build_alloca(typ, literal));

        let var = match val {
            Value::Numeric(_) => Var::Numeric(ptr),
            Value::Pending => Var::Pending,
            Value::Null => Var::Null,
            Value::String(_) => Var::String(ptr),
            Value::GlobalString(_) => Var::GlobalString(ptr),
            Value::Vec(_) => Var::Vec(ptr),
            Value::Bool(_) => Var::Bool(ptr),
            Value::Function { .. } => Var::Function(ptr),
        };

        match val {
            Value::Numeric(v)
            | Value::String(v)
            | Value::GlobalString(v)
            | Value::Vec(v)
            | Value::Bool(v)
            | Value::Function { val: v, .. } => {
                self.builder.create_store(v, &ptr);
            }
            _ => todo!(),
        };

        self.stack.last_mut().unwrap().set(literal, var);
    }

    fn remove_var(&mut self, literal: &str) {
        self.stack.last_mut().unwrap().remove(literal);
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
                return match v {
                    Var::Numeric(v)
                    | Var::String(v)
                    | Var::GlobalString(v)
                    | Var::Vec(v)
                    | Var::Bool(v)
                    | Var::Function(v) => Some(Value::Numeric(self.builder.build_load(&v, ""))),
                    _ => todo!(),
                }
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
            fpm,
            opt: true,
        }
    }
}
