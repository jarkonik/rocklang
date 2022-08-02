extern crate llvm_sys as llvm;

use std::error::Error;

use llvm_sys::{
    analysis::{LLVMVerifierFailureAction, LLVMVerifyFunction},
    core::LLVMGetParam,
};

use super::{LLVMError, Value};

#[derive(Debug, Clone, Copy)]
pub struct Function(pub *mut llvm::LLVMValue);

impl Function {
    pub fn get_param(&self, idx: u32) -> Value {
        Value::from(unsafe { LLVMGetParam(self.0, idx) })
    }

    pub fn verify_function(&self) -> Result<(), LLVMError> {
        let result = unsafe {
            LLVMVerifyFunction(self.0, LLVMVerifierFailureAction::LLVMPrintMessageAction)
        };
        if result == 0 {
            Ok(())
        } else {
            Err(LLVMError {})
        }
    }

    pub fn from(ptr: *mut llvm_sys::LLVMValue) -> Function {
        Function(ptr)
    }
}
