use crate::llvm;
use crate::parser;

#[derive(Debug, Clone)]
pub enum Value {
	Null,
	String(llvm::Value),
	GlobalString(llvm::Value),
	ConstNumeric(llvm::Value),
	Numeric(llvm::Value),
	Bool(llvm::Value),
	Function {
		val: llvm::Value,
		typ: llvm::Type,
		return_type: parser::Type,
	},
	Vec(llvm::Value),
	Pending,
}
