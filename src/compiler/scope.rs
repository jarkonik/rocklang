use crate::llvm::{Builder, Module};
use std::collections::HashMap;

use super::{CompilerResult, Value};

pub struct Scope {
    env: HashMap<String, Value>,
    references: Vec<Value>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            env: HashMap::new(),
            references: vec![],
        }
    }

    pub fn get(&self, literal: &str) -> Option<&Value> {
        self.env.get(literal)
    }

    pub fn set(&mut self, literal: &str, val: Value) {
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
                Value::Numeric(_) => unreachable!(),
                Value::Bool(_) => unreachable!(),
                Value::Function { .. } => unreachable!(),
                Value::Void => unreachable!(),
                Value::Break => unreachable!(),
                Value::Ptr(_) => unreachable!(),
            }
        }
        Ok(())
    }
}
