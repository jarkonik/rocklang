use crate::llvm;

#[derive(Debug, Clone, Copy)]
pub enum Value {
	Numeric(llvm::Value),
	NumericPtr(llvm::Value),
}
