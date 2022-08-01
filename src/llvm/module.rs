extern crate llvm_sys as llvm;

use core::panic;
use std::{ffi::CStr, fmt::Display, mem};

use libc::c_void;
use llvm::{
    core::{LLVMCreateModuleProviderForExistingModule, LLVMDumpModule, LLVMPrintModuleToString},
    execution_engine::{LLVMCreateExecutionEngineForModule, LLVMLinkInMCJIT},
    target::{LLVM_InitializeNativeAsmPrinter, LLVM_InitializeNativeTarget},
};
use llvm_sys::core::{LLVMAddFunction, LLVMAddGlobal, LLVMGetNamedFunction};

use super::{utils::c_str, Context, Engine, Function, Type, Value};

pub struct Module(*mut llvm::LLVMModule);

#[allow(dead_code)]
impl Module {
    pub fn new(name: &str, context: &Context) -> Self {
        context.create_module(name)
    }

    pub fn add_global(&self, typ: Type, name: &str) -> Value {
        Value::from(unsafe { LLVMAddGlobal(self.0, typ.0, c_str(name).as_ptr()) })
    }

    pub fn add_function(&self, name: &str, function_type: Type) -> Function {
        Function::from(unsafe { LLVMAddFunction(self.0, c_str(name).as_ptr(), function_type.0) })
    }

    pub fn get_function(&self, name: &str) -> Option<Function> {
        let fun = unsafe { LLVMGetNamedFunction(self.0, c_str(name).as_ptr()) };
        if fun.is_null() {
            None
        } else {
            Some(Function::from(fun))
        }
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        unsafe { LLVMDumpModule(self.0) }
    }

    pub fn from_context(ptr: *mut llvm::LLVMModule) -> Module {
        Module(ptr)
    }

    pub fn create_engine(&self) -> Engine {
        Engine::from(unsafe {
            let mut ee = mem::zeroed();
            let mut out = mem::zeroed();

            LLVMLinkInMCJIT();
            LLVM_InitializeNativeTarget();
            LLVM_InitializeNativeAsmPrinter();

            if LLVMCreateExecutionEngineForModule(&mut ee, self.0, &mut out) != 0 {
                panic!()
            };

            ee
        })
    }

    pub fn create_module_provider(&self) -> *mut llvm::LLVMModuleProvider {
        unsafe { LLVMCreateModuleProviderForExistingModule(self.0) }
    }
}

impl Display for Module {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let raw_ptr = unsafe { LLVMPrintModuleToString(self.0) };
        let str = unsafe { CStr::from_ptr(raw_ptr).to_str() };
        fmt.write_str(str.unwrap())?;
        unsafe { libc::free(raw_ptr as *mut c_void) };
        Ok(())
    }
}
