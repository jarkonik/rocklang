use crate::llvm;
use crate::parser;

#[derive(Debug, Clone, Copy)]
pub enum Var {
	Numeric(llvm::Value),
	Pending,
	Null,
	String(llvm::Value),
	Vec(llvm::Value),
	GlobalString(llvm::Value),
	Bool(llvm::Value),
	Function {
		val: llvm::Value,
		typ: llvm::Type,
		return_type: parser::Type,
	},
}
