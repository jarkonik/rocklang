use crate::llvm;
use std::collections::HashMap;

use super::Value;

pub struct Scope {
    env: HashMap<String, Value>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            env: HashMap::new(),
        }
    }

    pub fn get(&self, literal: &str) -> Option<&Value> {
        self.env.get(literal)
    }

    pub fn set(&mut self, literal: &str, val: Value) {
        self.env.insert(literal.to_string(), val);
    }
}
