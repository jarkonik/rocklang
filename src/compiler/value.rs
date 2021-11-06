use crate::llvm;

#[derive(Debug, Clone, Copy)]
pub enum Value {
	Numeric(llvm::Value),
}

#[derive(Debug, Clone, Copy)]
pub enum Ptr {
	Numeric(llvm::Value),
}
