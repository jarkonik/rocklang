mod frame;
mod value;

use crate::compiler::frame::Frame;
use crate::compiler::value::Value;
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
        todo!()
    }

    fn visit_numeric(&mut self, f: &f64) -> Value {
        todo!()
    }

    fn visit_conditional(&mut self, expr: &expression::Conditional) -> Value {
        todo!()
    }

    fn visit_assignment(&mut self, expr: &expression::Assignment) -> Value {
        todo!()
    }

    fn visit_unary(&mut self, _: &expression::Unary) -> Value {
        todo!()
    }

    fn visit_grouping(&mut self, _: &expression::Expression) -> Value {
        todo!()
    }

    fn visit_func_call(&mut self, expr: &expression::FuncCall) -> Value {
        todo!()
    }

    fn visit_while(&mut self, _: &expression::While) -> Value {
        todo!()
    }

    fn visit_identifier(&mut self, _: &str) -> Value {
        todo!()
    }

    fn visit_string(&mut self, _: &str) -> Value {
        todo!()
    }

    fn visit_bool(&mut self, _: &bool) -> Value {
        todo!()
    }

    fn visit_break(&mut self) -> Value {
        todo!()
    }

    fn visit_program(&mut self, _: parser::Program) -> Value {
        todo!()
    }

    fn visit_func_decl(&mut self, _: &expression::FuncDecl) -> Value {
        todo!()
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
        self.module.dump();
    }

    pub fn ir_string(&self) -> String {
        format!("{}", self.module)
    }

    fn set_var(&mut self, literal: &str, val: Value) {
        self.stack.last_mut().unwrap().set(literal, val);
    }

    fn remove_var(&mut self, literal: &str) {
        self.stack.last_mut().unwrap().remove(literal);
    }

    fn get_var(&mut self, literal: &str) -> Option<Value> {
        for frame in self.stack.iter().rev() {
            if let Some(v) = frame.get(literal) {
                return Some((*v).clone());
            };
        }
        None
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
