use crate::compiler::var::Var;
use crate::llvm;
use std::collections::HashMap;
use std::convert::TryInto;

pub struct Frame {
    env: HashMap<String, Var>,
    pub fun: llvm::Value,
}

impl Frame {
    pub fn new(fun: llvm::Value) -> Self {
        Frame {
            env: HashMap::new(),
            fun,
        }
    }

    pub fn get(&self, literal: &str) -> Option<&Var> {
        self.env.get(literal)
    }

    pub fn set(&mut self, literal: &str, val: Var) {
        self.env.insert(literal.to_string(), val);
    }

    #[allow(dead_code)]
    pub fn remove(&mut self, literal: &str) {
        self.env.remove(&literal.to_string());
    }

    pub fn dealloc(&self, context: &llvm::Context, builder: &llvm::Builder) {
        for val in self.env.values() {
            if let Var::Vec(v) = val {
                let fun_type = context.function_type(
                    context.void_type(),
                    &[context.double_type().pointer_type(0)],
                    false,
                );

                let fun_addr = stdlib::vecfree as usize;
                let ptr = context.const_u64_to_ptr(
                    context.const_u64(fun_addr.try_into().unwrap()),
                    fun_type.pointer_type(0),
                );
                builder.build_call(&ptr, &[builder.build_load(v, "")], "");
            }
        }
    }
}
