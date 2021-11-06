use crate::compiler::value::Ptr;
use crate::llvm;
use std::collections::HashMap;

pub struct Frame {
	env: HashMap<String, Ptr>,
	pub fun: llvm::Value,
}

impl Frame {
	pub fn new(fun: llvm::Value) -> Self {
		Frame {
			env: HashMap::new(),
			fun,
		}
	}

	pub fn get(&self, literal: &str) -> Option<&Ptr> {
		self.env.get(literal)
	}

	pub fn set(&mut self, literal: &str, val: Ptr) {
		self.env.insert(literal.to_string(), val);
	}

	pub fn remove(&mut self, literal: &str) {
		self.env.remove(&literal.to_string());
	}
}
