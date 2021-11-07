use crate::llvm;

#[derive(Debug, Clone, Copy)]
pub enum Var {
	Numeric(llvm::Value),
	Pending,
}
