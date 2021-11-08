use crate::llvm;

#[derive(Debug, Clone, Copy)]
pub enum Var {
	Numeric(llvm::Value),
	Pending,
	Null,
	String(llvm::Value),
	Vec(llvm::Value),
	GlobalString(llvm::Value),
	Bool(llvm::Value),
	Function(llvm::Value),
}
