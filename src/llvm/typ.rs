use llvm::core::LLVMPointerType;

extern crate llvm_sys as llvm;

#[derive(Debug, Clone, Copy)]
pub struct Type(pub *mut llvm::LLVMType);

impl Type {
    pub fn new(typ: *mut llvm::LLVMType) -> Self {
        Type(typ)
    }

    pub fn pointer_type(&self, address_space: u32) -> Type {
        Type(unsafe { LLVMPointerType(self.0, address_space) })
    }
}
