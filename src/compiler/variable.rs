use crate::{llvm, parser};

#[derive(Debug, Clone, Copy)]
pub enum Variable {
    String(llvm::Value),
    Numeric(llvm::Value),
    Bool(llvm::Value),
    Function {
        val: llvm::Function,
        typ: llvm::Type,
        return_type: parser::Type,
    },
    Vec(llvm::Value),
    Ptr(llvm::Value),
}
