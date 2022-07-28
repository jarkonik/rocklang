extern crate llvm_sys as llvm;
#[allow(dead_code)]
pub struct FunctionType(*mut llvm::LLVMType);
