use std::mem;

use llvm::execution_engine::{
    LLVMDisposeExecutionEngine, LLVMGetFunctionAddress, LLVMOpaqueExecutionEngine,
};

use super::{utils::c_str, Module};

extern crate llvm_sys as llvm;

pub struct Engine(*mut llvm::execution_engine::LLVMOpaqueExecutionEngine);

impl Drop for Engine {
    fn drop(&mut self) {
        unsafe { LLVMDisposeExecutionEngine(self.0) }
    }
}

impl Engine {
    pub fn new(module: &Module) -> Self {
        module.create_engine()
    }

    pub fn from(ptr: *mut LLVMOpaqueExecutionEngine) -> Self {
        Engine(ptr)
    }

    pub fn call(&self, name: &str) {
        unsafe {
            let addr = LLVMGetFunctionAddress(self.0, c_str(name).as_ptr());
            let f: extern "C" fn() = mem::transmute(addr);
            f();
        }
    }
}
