use crate::llvm;
use crate::parser;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Null,
    String(llvm::Value),
    ConstString(llvm::Value),
    Numeric(llvm::Value),
    Bool(llvm::Value),
    Function {
        val: llvm::Function,
        typ: llvm::Type,
        return_type: parser::Type,
    },
    Vec(llvm::Value),
    Break,
    Ptr(llvm::Value),
}
