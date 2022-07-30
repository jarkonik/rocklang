mod assignment;
mod binary;
mod conditional;
mod func_call;
mod numeric;
mod program;
mod scope;
mod string;
mod unary;
mod value;

use crate::expression;
use crate::expression::Expression;
use crate::expression::FuncDecl;
use crate::llvm;
use crate::llvm::BasicBlock;
use crate::llvm::Function;
use crate::llvm::PassManager;
use crate::parser;
use crate::parser::Program;
use crate::visitor::ProgramVisitor;
use crate::visitor::Visitor;
use std::convert::TryInto;
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
    fn visit_grouping(&mut self, expr: &expression::Expression) -> CompilerResult<Value> {
        todo!()
    }

    fn visit_while(&mut self, expr: &expression::While) -> CompilerResult<Value> {
        todo!()
    }

    fn visit_identifier(&mut self, expr: &str) -> CompilerResult<Value> {
        todo!()
    }

    fn visit_bool(&mut self, expr: &bool) -> CompilerResult<Value> {
        todo!()
    }

    fn visit_break(&mut self) -> CompilerResult<Value> {
        todo!()
    }
    fn visit_func_decl(&mut self, body: &expression::FuncDecl) -> CompilerResult<Value> {
        todo!()
    }

    fn visit_load(&mut self, name: &str) -> CompilerResult<Value> {
        todo!()
    }

    fn visit_extern(&mut self, name: &expression::Extern) -> CompilerResult<Value> {
        todo!()
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
    fn get_var(&self, name: &str) -> Option<&Value> {
        self.scopes.last().unwrap().get(name)
    }

    fn set_var(&mut self, name: &str, val: Value) {
        self.scopes.last_mut().unwrap().set(name, val);
    }

    fn build_function(&mut self, fun_compiler_val: Value, expr: FuncDecl) -> CompilerResult<()> {
        let fun = match fun_compiler_val {
            Value::Function { val, .. } => val,
            _ => panic!(),
        };

        let curr = self.builder.get_insert_block();

        let block = self.context.append_basic_block(&fun, "entry");
        self.builder.position_builder_at_end(&block);

        self.scopes.push(Scope::new());

        for (i, param) in expr.params.iter().enumerate() {
            self.set_var(
                param.name.as_str(),
                match param.typ {
                    parser::Type::Numeric => todo!(),
                    parser::Type::Vector => todo!(),
                    parser::Type::Null => todo!(),
                    parser::Type::Function => todo!(),
                    parser::Type::Ptr => todo!(),
                    parser::Type::String => todo!(),
                },
            )
        }

        let mut last_val = Value::Null;
        for stmt in expr.body.clone() {
            last_val = self.walk(&stmt)?;
        }

        self.scopes.pop().unwrap();

        let ret_val = match last_val {
            Value::String(_) => todo!(),
            Value::Bool(_) => todo!(),
            Value::Function {
                val,
                typ,
                return_type,
            } => todo!(),
            Value::Vec(_) => todo!(),
            Value::Break => todo!(),
            Value::Ptr(_) => todo!(),
            Value::Null => todo!(),
            Value::Numeric(_) => todo!(),
        };

        match ret_val {
            Some(v) => self.builder.build_ret(v),
            None => self.builder.build_ret_void(),
        };

        self.builder.position_builder_at_end(&curr);

        self.verify_function(fun);

        if self.optimization {
            self.pass_manager.run(&fun);
        };
        Ok(())
    }

    fn verify_function(&mut self, fun: Function) {
        fun.verify_function().unwrap_or_else(|_x| {
            println!("IR Dump:");
            self.dump_ir();
            panic!("Function verification failed")
        });
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
                return_type: parser::Type::Null,
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
        let hello = self.module.add_function("string", fun_type);
        scope.set(
            "string",
            Value::Function {
                val: hello,
                typ: fun_type,
                return_type: parser::Type::String,
            },
        );
    }

    fn init_builtins(&mut self) {
        self.init_print_fn();
        self.init_string_fn();

        let scope = self.scopes.last_mut().unwrap();
        self.context.add_symbol(
            "release_string_reference",
            stdlib::release_string_reference as *mut c_void,
        );
        let fun_type = self.context.function_type(
            self.context.void_type(),
            &[self.context.void_type().pointer_type(0)],
            false,
        );
        let fun = self
            .module
            .add_function("release_string_reference", fun_type);
        scope.set(
            "release_string_reference",
            Value::Function {
                val: fun,
                typ: fun_type,
                return_type: parser::Type::String,
            },
        );

        self.context.add_symbol(
            "string_from_c_string",
            stdlib::string_from_c_string as *mut c_void,
        );
        let fun_type = self.context.function_type(
            self.context.void_type().pointer_type(0),
            &[self.context.i8_type().pointer_type(0)],
            false,
        );
        let fun = self.module.add_function("string_from_c_string", fun_type);
        scope.set(
            "string_from_c_string",
            Value::Function {
                val: fun,
                typ: fun_type,
                return_type: parser::Type::String,
            },
        );
    }
}
