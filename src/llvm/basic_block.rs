use llvm::core::LLVMGetBasicBlockParent;

use super::Function;

extern crate llvm_sys as llvm;

#[derive(Clone, Copy, Debug)]
pub struct BasicBlock(pub *mut llvm::LLVMBasicBlock);

impl BasicBlock {
    pub fn new(block: *mut llvm::LLVMBasicBlock) -> Self {
        BasicBlock(block)
    }

    pub fn get_parent(&self) -> Function {
        Function::from(unsafe { LLVMGetBasicBlockParent(self.0) })
    }
}
