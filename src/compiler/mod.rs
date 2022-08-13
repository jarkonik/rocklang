mod assignment;
mod binary;
mod bool;
mod break_visitor;
mod conditional;
mod extern_visitor;
mod func_call;
mod func_decl_vistor;
mod grouping;
mod identifier;
mod load;
mod numeric;
mod program;
mod scope;
mod string;
mod unary;
mod utils;
mod value;
mod variable;
mod while_visitor;

use crate::expression;
use crate::expression::Expression;
use crate::llvm;
use crate::llvm::Builder;
use crate::llvm::Context;
use crate::llvm::Function;
use crate::llvm::Module;
use crate::llvm::Type;
use crate::parser;
use crate::parser::Program;
use crate::parser::Span;
use crate::visitor::*;
use std::collections::HashMap;
use std::error::Error;
use std::ffi::c_void;
use std::fmt;

use self::scope::Scope;
pub use self::value::Value;
use self::variable::Variable;

#[derive(Clone, Debug)]
pub enum CompilerError {
    VoidAssignment,
    NonIdentifierAssignment {
        span: Span,
    },
    TypeError {
        expected: parser::Type,
        actual: parser::Type,
        span: Span,
    },
    EngineInitError,
    UndefinedIdentifier(String),
    LLVMError(String),
    LoadLibaryError(String),
    WrongOperator {
        expected: expression::Operator,
        actual: expression::Operator,
        span: Span,
    },
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            CompilerError::TypeError {
                expected,
                actual,
                span,
            } => {
                format!(
                    "type error expected {}, but got {} at {}",
                    expected, actual, span
                )
            }
            CompilerError::EngineInitError => "engine init error".to_string(),
            CompilerError::UndefinedIdentifier(x) => {
                format!("undefined identifier {}", x)
            }
            CompilerError::LLVMError(err) => format!("llvm error: {}", err),
            CompilerError::LoadLibaryError(lib) => format!("error loading {}", lib),
            CompilerError::VoidAssignment => "void assignment".to_string(),
            CompilerError::NonIdentifierAssignment { span } => {
                format!("non identifier assignment at {}", span)
            }
            CompilerError::WrongOperator {
                expected,
                actual,
                span,
            } => format!(
                "wrong operator, expected {:#?}, but got {:#?} at {}",
                expected, actual, span
            ),
        };
        write!(f, "{}", msg)
    }
}

impl Error for CompilerError {}

type CompilerResult<T> = Result<T, CompilerError>;

const MAIN_FUNCTION: &str = "main";

pub trait Compile: ProgramVisitor<CompilerResult<Value>> {
    fn compile(&mut self) -> CompilerResult<Value>;
}

pub struct Compiler {
    after_loop_blocks: Vec<llvm::BasicBlock>,
    maybe_orphaned: Vec<Value>,
    program: Program,
    engine: llvm::Engine,
    context: llvm::Context,
    module: llvm::Module,
    builder: llvm::Builder,
    pass_manager: llvm::PassManager,
    optimization: bool,
    scopes: Vec<Scope>,
    builtins: HashMap<String, Variable>,
}

impl Visitor<CompilerResult<Value>> for Compiler {
    fn walk(&mut self, node: &crate::expression::Node) -> CompilerResult<Value> {
        let span = node.span.clone();
        match &node.expression {
            Expression::Binary(expr) => self.visit_binary(expr, span),
            Expression::Unary(expr) => self.visit_unary(expr, span),
            Expression::FuncCall(expr) => self.visit_func_call(expr, span),
            Expression::Numeric(expr) => self.visit_numeric(expr),
            Expression::Assignment(expr) => self.visit_assignment(expr, span),
            Expression::Identifier(expr) => self.visit_identifier(expr),
            Expression::Conditional(expr) => self.visit_conditional(expr, span),
            Expression::String(expr) => self.visit_string(expr),
            Expression::Bool(expr) => self.visit_bool(expr),
            Expression::Break => self.visit_break(),
            Expression::While(expr) => self.visit_while(expr, span),
            Expression::FuncDecl(expr) => self.visit_func_decl(expr),
            Expression::Load(expr) => self.visit_load(expr),
            Expression::Extern(expr) => self.visit_extern(expr),
            Expression::Grouping(expr) => self.visit_grouping(expr),
        }
    }
}

impl Compile for Compiler {
    fn compile(&mut self) -> CompilerResult<Value> {
        self.visit_program(self.program.clone())
    }
}

impl Compiler {
    fn verify_function(&mut self, fun: Function) -> CompilerResult<()> {
        if fun.verify_function().is_ok() {
            Ok(())
        } else {
            println!("{}", self.ir_string());
            Err(CompilerError::LLVMError(self.ir_string()))
        }
    }

    pub fn dump_ir(&self) {
        println!("{}", self.module);
    }

    pub fn ir_string(&self) -> String {
        format!("{}", self.module)
    }

    pub fn run(&self) {
        self.engine.call(MAIN_FUNCTION);
    }

    pub fn turn_off_optimization(&mut self) {
        self.optimization = false;
    }

    pub fn new(program: Program) -> CompilerResult<Self> {
        let context = llvm::Context::new();
        let module = llvm::Module::new("main", &context);
        let builder = llvm::Builder::new(&context);
        let engine = match llvm::Engine::new(&module) {
            Ok(e) => e,
            Err(_) => Err(CompilerError::EngineInitError {})?,
        };
        let pass_manager = llvm::PassManager::new(&module);

        Ok(Compiler {
            after_loop_blocks: Vec::new(),
            maybe_orphaned: Vec::new(),
            builtins: HashMap::new(),
            scopes: vec![],
            program,
            context,
            module,
            builder,
            engine,
            pass_manager,
            optimization: true,
        })
    }

    fn init_builtin(&mut self, name: &str, typ: Type, fun: *mut c_void, return_type: parser::Type) {
        self.context.add_symbol(name, fun);
        let val = self.module.add_function(name, typ);
        self.builtins.insert(
            name.to_string(),
            Variable::Function {
                val,
                typ,
                return_type,
            },
        );
    }

    fn init_builtins(&mut self) {
        let string_type = self.context.function_type(
            self.context.void_type().pointer_type(0),
            &[self.context.double_type()],
            false,
        );
        self.init_builtin(
            "string",
            string_type,
            stdlib::string as *mut c_void,
            parser::Type::String,
        );

        let print_type = self.context.function_type(
            self.context.void_type(),
            &[self.context.void_type().pointer_type(0)],
            false,
        );
        self.init_builtin(
            "print",
            print_type,
            stdlib::print as *mut c_void,
            parser::Type::Void,
        );

        self.init_builtin(
            "release_string_reference",
            self.context.function_type(
                self.context.void_type(),
                &[self.context.void_type().pointer_type(0)],
                false,
            ),
            stdlib::release_string_reference as *mut c_void,
            parser::Type::Void,
        );

        self.init_builtin(
            "inc_string_reference",
            self.context.function_type(
                self.context.void_type(),
                &[self.context.void_type().pointer_type(0)],
                false,
            ),
            stdlib::inc_string_reference as *mut c_void,
            parser::Type::Void,
        );

        self.init_builtin(
            "inc_vec_reference",
            self.context.function_type(
                self.context.void_type(),
                &[self.context.void_type().pointer_type(0)],
                false,
            ),
            stdlib::inc_vec_reference as *mut c_void,
            parser::Type::Void,
        );

        self.init_builtin(
            "release_vec_reference",
            self.context.function_type(
                self.context.void_type(),
                &[self.context.void_type().pointer_type(0)],
                false,
            ),
            stdlib::release_vec_reference as *mut c_void,
            parser::Type::Void,
        );

        self.init_builtin(
            "c_string_from_string",
            self.context.function_type(
                self.context.i8_type().pointer_type(0),
                &[self.context.void_type().pointer_type(0)],
                false,
            ),
            stdlib::c_string_from_string as *mut c_void,
            parser::Type::CString,
        );

        self.init_builtin(
            "string_from_c_string",
            self.context.function_type(
                self.context.void_type().pointer_type(0),
                &[self.context.i8_type().pointer_type(0)],
                false,
            ),
            stdlib::string_from_c_string as *mut c_void,
            parser::Type::String,
        );

        let vec_new_type =
            self.context
                .function_type(self.context.void_type().pointer_type(0), &[], false);
        self.init_builtin(
            "vec_new",
            vec_new_type,
            stdlib::vec_new as *mut c_void,
            parser::Type::Vector,
        );

        let vec_set_type = self.context.function_type(
            self.context.void_type(),
            &[
                self.context.void_type().pointer_type(0),
                self.context.double_type(),
                self.context.double_type(),
            ],
            false,
        );
        self.init_builtin(
            "vec_set",
            vec_set_type,
            stdlib::vec_set as *mut c_void,
            parser::Type::Void,
        );

        let vec_get_type = self.context.function_type(
            self.context.double_type(),
            &[
                self.context.void_type().pointer_type(0),
                self.context.double_type(),
            ],
            false,
        );
        self.init_builtin(
            "vec_get",
            vec_get_type,
            stdlib::vec_get as *mut c_void,
            parser::Type::Numeric,
        );

        let sqrt_type = self.context.function_type(
            self.context.double_type(),
            &[self.context.double_type()],
            false,
        );
        let val = self.module.add_function("sqrt", sqrt_type);
        self.builtins.insert(
            "sqrt".to_string(),
            Variable::Function {
                val,
                typ: sqrt_type,
                return_type: parser::Type::Numeric,
            },
        );
    }

    fn set_param(&mut self, name: &str, val: Value) {
        self.scopes.last_mut().unwrap().set_param(name, val);
    }

    fn get_param(&self, expr: &str) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get_param(expr) {
                return Some(*val);
            }
        }
        None
    }
}

trait LLVMCompiler: Visitor<CompilerResult<Value>> {
    fn builder(&self) -> &Builder;
    fn context(&self) -> &Context;
    fn module(&self) -> &Module;
    fn enter_scope(&mut self);
    fn exit_scope(&mut self) -> CompilerResult<()>;
    fn after_loop_blocks(&self) -> &Vec<llvm::BasicBlock>;
    fn get_var(&self, name: &str) -> Option<Variable>;
    fn get_builtin(&self, name: &str) -> Option<Variable>;
    fn track_maybe_orphaned(&mut self, val: Value);
    fn release_maybe_orphaned(&mut self);
    fn set_var(&mut self, name: &str, val: Variable);
    fn build_function(
        &mut self,
        fun_compiler_val: Value,
        expr: &expression::FuncDecl,
    ) -> Result<(), CompilerError>;
}

impl LLVMCompiler for Compiler {
    fn builder(&self) -> &Builder {
        &self.builder
    }

    fn context(&self) -> &Context {
        &self.context
    }

    fn module(&self) -> &Module {
        &self.module
    }

    fn enter_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    fn exit_scope(&mut self) -> CompilerResult<()> {
        let scope = self.scopes.pop().unwrap();
        self.release_maybe_orphaned();
        scope.release_references(self.context(), self.module(), self.builder())
    }

    fn get_var(&self, name: &str) -> Option<Variable> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Some(*val);
            }
        }
        None
    }

    fn get_builtin(&self, name: &str) -> Option<Variable> {
        self.builtins.get(name).copied()
    }

    fn track_maybe_orphaned(&mut self, val: Value) {
        self.maybe_orphaned.push(val);
    }

    fn release_maybe_orphaned(&mut self) {
        while let Some(val) = self.maybe_orphaned.pop() {
            match val {
                Value::Void => todo!(),
                Value::String(v) => {
                    let release = self
                        .module
                        .get_function("release_string_reference")
                        .unwrap();
                    self.builder.build_call(&release, &[v], "");
                }
                Value::Numeric(_) => todo!(),
                Value::Bool(_) => todo!(),
                Value::Function { .. } => todo!(),
                Value::Vec(v) => {
                    let release = self.module.get_function("release_vec_reference").unwrap();
                    self.builder.build_call(&release, &[v], "");
                }
                Value::Break => todo!(),
                Value::Ptr(_) => todo!(),
                Value::CString(_) => todo!(),
            }
        }
    }

    fn set_var(&mut self, name: &str, val: Variable) {
        self.scopes.last_mut().unwrap().set(name, val);
    }

    fn build_function(
        &mut self,
        fun_compiler_val: Value,
        expr: &expression::FuncDecl,
    ) -> Result<(), CompilerError> {
        let fun = match fun_compiler_val {
            Value::Function { val, .. } => val,
            Value::Void => todo!(),
            Value::String(_) => todo!(),
            Value::Numeric(_) => todo!(),
            Value::Bool(_) => todo!(),
            Value::Vec(_) => todo!(),
            Value::Break => todo!(),
            Value::Ptr(_) => todo!(),
            Value::CString(_) => todo!(),
        };

        let curr = self.builder.get_insert_block();

        let block = self.context.append_basic_block(&fun, "entry");
        self.builder.position_builder_at_end(&block);

        self.enter_scope();

        for (i, param) in expr.params.iter().enumerate() {
            let val = fun.get_param(i.try_into().unwrap());

            let val = match param.typ {
                parser::Type::String => {
                    let release = self.module.get_function("inc_vec_reference").unwrap();
                    self.builder.build_call(&release, &[val], "");

                    Value::String(val)
                }
                parser::Type::Numeric => Value::Numeric(val),
                parser::Type::Bool => Value::Bool(val),
                parser::Type::Vector => {
                    let release = self.module.get_function("inc_vec_reference").unwrap();
                    self.builder.build_call(&release, &[val], "");

                    Value::Vec(val)
                }
                parser::Type::Void => todo!(),
                parser::Type::Function => todo!(),
                parser::Type::Ptr => todo!(),
                parser::Type::CString => todo!(),
            };
            self.set_param(param.name.as_str(), val);
        }

        let mut last_val = Value::Void;

        for stmt in expr.body.clone() {
            self.release_maybe_orphaned();
            last_val = self.walk(&stmt)?;
        }

        let ret_val = match last_val {
            Value::Void => None,
            Value::Numeric(n) => Some(n),
            Value::Vec(n) => {
                let release = self.module.get_function("inc_vec_reference").unwrap();
                self.builder.build_call(&release, &[n], "");
                Some(n)
            }
            Value::String(n) => {
                let release = self.module.get_function("inc_string_reference").unwrap();
                self.builder.build_call(&release, &[n], "");

                Some(n)
            }
            Value::Bool(_) => todo!(),
            Value::Function { .. } => todo!(),
            Value::Break => todo!(),
            Value::Ptr(_) => todo!(),
            Value::CString(_) => todo!(),
        };

        self.exit_scope()?;

        match ret_val {
            Some(v) => self.builder.build_ret(v),
            None => self.builder.build_ret_void(),
        };

        self.builder.position_builder_at_end(&curr);

        self.verify_function(fun)?;

        if self.optimization {
            self.pass_manager.run(&fun);
        }

        Ok(())
    }

    fn after_loop_blocks(&self) -> &Vec<llvm::BasicBlock> {
        &self.after_loop_blocks
    }
}
