use crate::llvm::{self, Builder, Context, Module};
use std::{collections::HashMap, ffi::c_void};

use super::Value;

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

    pub fn release_references(&self, module: &Module, context: &Context, builder: &Builder) {
        for reference in self.references.iter() {
            match reference {
                Value::Null => todo!(),
                Value::String(val) => {
                    context.add_symbol(
                        "release_string_reference",
                        stdlib::release_string_reference as *mut c_void,
                    );
                    let fun_type = context.function_type(
                        context.void_type(),
                        &[context.void_type().pointer_type(0)],
                        false,
                    );
                    let fun = module.add_function("release_string_reference", fun_type);
                    builder.build_call(&fun, &[*val], "");
                }
                Value::ConstString(_) => todo!(),
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
    }
}
