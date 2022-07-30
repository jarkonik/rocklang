use llvm::{
    core::{LLVMSetInitializer},
};

extern crate llvm_sys as llvm;

trait LLVMValue {
    fn value(&self) -> *mut llvm::LLVMValue;
}

trait Initializer<T: LLVMValue = Self>: LLVMValue {
    fn set_initializer(&self, value: T) {
        unsafe {
            LLVMSetInitializer(self.value(), value.value());
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct I32(*mut llvm::LLVMValue);

#[derive(Debug, Clone, Copy)]
pub struct Value(pub *mut llvm::LLVMValue);

impl From<*mut llvm::LLVMValue> for Value {
    fn from(ptr: *mut llvm::LLVMValue) -> Self {
        Value(ptr)
    }
}

impl From<*mut llvm::LLVMValue> for I32 {
    fn from(ptr: *mut llvm::LLVMValue) -> Self {
        I32(ptr)
    }
}

impl LLVMValue for Value {
    fn value(&self) -> *mut llvm::LLVMValue {
        self.0
    }
}

impl LLVMValue for I32 {
    fn value(&self) -> *mut llvm::LLVMValue {
        self.0
    }
}

impl Initializer for I32 {}
impl Initializer for Value {}
