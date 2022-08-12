use crate::llvm::{Builder, Module};
use std::collections::HashMap;

use super::{variable::Variable, CompilerResult, Value};

pub struct Scope {
    env: HashMap<String, Variable>,
    references: Vec<Value>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            env: HashMap::new(),
            references: vec![],
        }
    }

    pub fn get(&self, literal: &str) -> Option<&Variable> {
        self.env.get(literal)
    }

    pub fn set(&mut self, literal: &str, val: Variable) {
        self.env.insert(literal.to_string(), val);
    }

    pub fn track_reference(&mut self, value: Value) {
        self.references.push(value);
    }

    pub fn release_references(&self, module: &Module, builder: &Builder) -> CompilerResult<()> {
        for reference in self.references.iter().rev() {
            match reference {
                Value::String(val) => {
                    let release = module.get_function("release_string_reference").unwrap();
                    builder.build_call(&release, &[*val], "");
                }
                Value::Vec(val) => {
                    let release = module.get_function("release_vec_reference").unwrap();
                    builder.build_call(&release, &[*val], "");
                }
                Value::Numeric(_) => unimplemented!(),
                Value::Bool(_) => unimplemented!(),
                Value::Function { .. } => unimplemented!(),
                Value::Void => unimplemented!(),
                Value::Break => unimplemented!(),
                Value::Ptr(_) => unimplemented!(),
            }
        }
        Ok(())
    }
}
