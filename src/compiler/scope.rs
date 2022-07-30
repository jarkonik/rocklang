use crate::llvm::{Builder, Context, Module};
use std::{collections::HashMap};

use super::{CompilerError, CompilerResult, Value};

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

    pub fn release_references(
        &self,
        module: &Module,
        context: &Context,
        builder: &Builder,
    ) -> CompilerResult<()> {
        for reference in self.references.iter() {
            match reference {
                Value::Null => todo!(),
                Value::String(val) => {
                    let fun = if let Value::Function { val, .. } =
                        self.get("release_string_reference").unwrap()
                    {
                        val
                    } else {
                        Err(CompilerError::TypeError)?
                    };
                    builder.build_call(&fun, &[*val], "");
                }
                Value::Numeric(_) => todo!(),
                Value::Bool(_) => todo!(),
                Value::Function {
                    val,
                    typ,
                    return_type,
                } => todo!(),
                Value::Vec(_) => todo!(),
                Value::Break => todo!(),
                Value::Ptr(_) => todo!(),
                Value::Ptr(_) => todo!(),
            }
        }
        Ok(())
    }
}
