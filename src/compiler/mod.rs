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
use crate::visitor::*;
use std::error::Error;
use std::ffi::c_void;
use std::fmt;

use self::scope::Scope;
pub use self::value::Value;
use self::variable::Variable;

#[derive(Clone, Debug)]
pub enum CompilerError {
    TypeError,
    EngineInitError,
    UndefinedIdentifier(String),
    LLVMError(String),
    LoadLibaryError(String),
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Input error")
    }
}

impl Error for CompilerError {}

type CompilerResult<T> = Result<T, CompilerError>;

const MAIN_FUNCTION: &str = "main";

pub trait Compile: ProgramVisitor<CompilerResult<Value>> {
    fn compile(&mut self) -> CompilerResult<Value>;
}

pub struct Compiler {
    program: Program,
    engine: llvm::Engine,
    context: llvm::Context,
    module: llvm::Module,
    builder: llvm::Builder,
    pass_manager: llvm::PassManager,
    optimization: bool,
    scopes: Vec<Scope>,
}

impl Visitor<CompilerResult<Value>> for Compiler {
    fn walk(&mut self, expr: &crate::expression::Expression) -> CompilerResult<Value> {
        match expr {
            Expression::Binary(expr) => self.visit_binary(expr),
            Expression::Unary(expr) => self.visit_unary(expr),
            Expression::FuncCall(expr) => self.visit_func_call(expr),
            Expression::Numeric(expr) => self.visit_numeric(expr),
            Expression::Assignment(expr) => self.visit_assignment(expr),
            Expression::Identifier(expr) => self.visit_identifier(expr),
            Expression::Conditional(expr) => self.visit_conditional(expr),
            Expression::String(expr) => self.visit_string(expr),
            Expression::Bool(expr) => self.visit_bool(expr),
            Expression::Break => self.visit_break(),
            Expression::While(expr) => self.visit_while(expr),
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

    fn init_builtin(&self, name: &str, function_type: Type, fun: *mut c_void) -> Function {
        self.context.add_symbol(name, fun);
        self.module.add_function(name, function_type)
    }

    fn init_builtins(&mut self) {
        let string_type = self.context.function_type(
            self.context.void_type().pointer_type(0),
            &[self.context.double_type()],
            false,
        );
        let string = self.init_builtin("string", string_type, stdlib::string as *mut c_void);
        self.set_var(
            "string",
            Variable::Function {
                val: string,
                typ: string_type,
                return_type: parser::Type::String,
            },
        );

        let print_type = self.context.function_type(
            self.context.void_type(),
            &[self.context.void_type().pointer_type(0)],
            false,
        );
        let print = self.init_builtin("print", print_type, stdlib::print as *mut c_void);
        self.set_var(
            "print",
            Variable::Function {
                val: print,
                typ: print_type,
                return_type: parser::Type::Void,
            },
        );

        self.init_builtin(
            "release_string_reference",
            self.context.function_type(
                self.context.void_type(),
                &[self.context.void_type().pointer_type(0)],
                false,
            ),
            stdlib::release_string_reference as *mut c_void,
        );

        self.init_builtin(
            "release_vec_reference",
            self.context.function_type(
                self.context.void_type(),
                &[self.context.void_type().pointer_type(0)],
                false,
            ),
            stdlib::release_vec_reference as *mut c_void,
        );

        self.init_builtin(
            "string_from_c_string",
            self.context.function_type(
                self.context.void_type().pointer_type(0),
                &[self.context.i8_type().pointer_type(0)],
                false,
            ),
            stdlib::string_from_c_string as *mut c_void,
        );

        let vec_new_type =
            self.context
                .function_type(self.context.void_type().pointer_type(0), &[], false);
        let vec_new = self.init_builtin("vec_new", vec_new_type, stdlib::vec_new as *mut c_void);
        self.set_var(
            "vec_new",
            Variable::Function {
                val: vec_new,
                typ: vec_new_type,
                return_type: parser::Type::Vector,
            },
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
        let vec_set = self.init_builtin("vec_set", vec_set_type, stdlib::vec_set as *mut c_void);
        self.set_var(
            "vec_set",
            Variable::Function {
                val: vec_set,
                typ: vec_set_type,
                return_type: parser::Type::Void,
            },
        );
    }
}

trait LLVMCompiler: Visitor<CompilerResult<Value>> {
    fn builder(&self) -> &Builder;
    fn context(&self) -> &Context;
    fn module(&self) -> &Module;
    fn enter_scope(&mut self);
    fn exit_scope(&mut self) -> CompilerResult<()>;
    fn get_var(&self, name: &str) -> CompilerResult<Variable>;
    fn track_reference(&mut self, val: Value);
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

    fn get_var(&self, name: &str) -> CompilerResult<Variable> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Ok(*val);
            }
        }
        Err(CompilerError::UndefinedIdentifier(name.to_string()))
    }

    fn set_var(&mut self, name: &str, val: Variable) {
        self.scopes.last_mut().unwrap().set(name, val);
    }

    fn exit_scope(&mut self) -> CompilerResult<()> {
        let scope = self.scopes.pop().unwrap();
        scope.release_references(self.module(), self.builder())
    }

    fn track_reference(&mut self, val: Value) {
        self.scopes.last_mut().unwrap().track_reference(val);
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
        };

        let curr = self.builder.get_insert_block();

        let block = self.context.append_basic_block(&fun, "entry");
        self.builder.position_builder_at_end(&block);

        self.enter_scope();

        for (i, param) in expr.params.iter().enumerate() {
            self.set_var(
                param.name.as_str(),
                match param.typ {
                    parser::Type::Vector => Variable::Vec(fun.get_param(i.try_into().unwrap())),
                    parser::Type::Numeric => {
                        Variable::Numeric(fun.get_param(i.try_into().unwrap()))
                    }
                    parser::Type::Ptr => Variable::Ptr(fun.get_param(i.try_into().unwrap())),
                    parser::Type::String => Variable::String(fun.get_param(i.try_into().unwrap())),
                    parser::Type::Void => Err(CompilerError::TypeError)?,
                    parser::Type::Function => todo!(),
                    parser::Type::Bool => todo!(),
                },
            )
        }

        let mut last_val = Value::Void;

        for stmt in expr.body.clone() {
            last_val = self.walk(&stmt)?;
        }

        let ret_val = match last_val {
            Value::Void => None,
            Value::Numeric(n) => Some(n),
            Value::Vec(n) => Some(n),
            Value::String(_) => todo!(),
            Value::Bool(_) => todo!(),
            Value::Function { .. } => todo!(),
            Value::Break => todo!(),
            Value::Ptr(_) => todo!(),
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
}
