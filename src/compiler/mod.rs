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
mod while_visitor;

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

#[derive(Clone, Debug)]
pub enum CompilerError {
    UnkownError,
    TypeError,
    UndefinedIdentifier(String),
    LLVMError(String),
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

impl Default for Compiler {
    fn default() -> Self {
        Compiler::new(Program::default())
    }
}

impl Compiler {
    fn verify_function(&mut self, fun: Function) -> CompilerResult<()> {
        if let Ok(fun) = fun.verify_function() {
            Ok(())
        } else {
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

    pub fn new(program: Program) -> Self {
        let context = llvm::Context::new();
        let module = llvm::Module::new("main", &context);
        let builder = llvm::Builder::new(&context);
        let engine = llvm::Engine::new(&module);
        let pass_manager = llvm::PassManager::new(&module);

        Compiler {
            scopes: vec![],
            program,
            context,
            module,
            builder,
            engine,
            pass_manager,
            optimization: true,
        }
    }

    fn init_print_fn(&mut self) {
        let scope = self.scopes.last_mut().unwrap();

        self.context
            .add_symbol("print", stdlib::print as *mut c_void);
        let fun_type = self.context.function_type(
            self.context.void_type(),
            &[self.context.void_type().pointer_type(0)],
            false,
        );
        let print = self.module.add_function("print", fun_type);
        scope.set(
            "print",
            Value::Function {
                val: print,
                typ: fun_type,
                return_type: parser::Type::Void,
            },
        );
    }

    fn init_string_fn(&mut self) {
        let scope = self.scopes.last_mut().unwrap();

        self.context
            .add_symbol("string", stdlib::string as *mut c_void);
        let fun_type = self.context.function_type(
            self.context.void_type().pointer_type(0),
            &[self.context.double_type()],
            false,
        );
        let string_fun = self.module.add_function("string", fun_type);
        scope.set(
            "string",
            Value::Function {
                val: string_fun,
                typ: fun_type,
                return_type: parser::Type::String,
            },
        );
    }

    fn init_builtin(&self, name: &str, function_type: Type, fun: *mut c_void) -> Function {
        self.context.add_symbol(name, fun);
        self.module.add_function(name, function_type)
    }

    fn init_builtins(&mut self) {
        let print_type = self.context.function_type(
            self.context.void_type(),
            &[self.context.void_type().pointer_type(0)],
            false,
        );
        let print = self.init_builtin("print", print_type, stdlib::print as *mut c_void);
        self.set_var(
            "print",
            Value::Function {
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
            Value::Function {
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
            Value::Function {
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
    fn get_var(&self, name: &str) -> CompilerResult<Value>;
    fn track_reference(&mut self, val: Value);
    fn set_var(&mut self, name: &str, val: Value);
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

    fn get_var(&self, name: &str) -> CompilerResult<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Ok(*val);
            }
        }
        Err(CompilerError::UndefinedIdentifier(name.to_string()))
    }

    fn set_var(&mut self, name: &str, val: Value) {
        self.scopes.last_mut().unwrap().set(name, val);
    }

    fn exit_scope(&mut self) -> CompilerResult<()> {
        let scope = self.scopes.pop().unwrap();
        scope.release_references(self.module(), self.builder())
    }

    fn track_reference(&mut self, val: Value) {
        self.scopes.last_mut().unwrap().track_reference(val);
    }
}
