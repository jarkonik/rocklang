extern crate llvm_sys as llvm;

use llvm::core::LLVMDisposePassManager;
use llvm_sys::{
    core::{
        LLVMCreateFunctionPassManager, LLVMInitializeFunctionPassManager,
        LLVMRunFunctionPassManager,
    },
    transforms::{scalar::*, util::LLVMAddPromoteMemoryToRegisterPass},
};

use super::{Function, Module, Value};

pub struct PassManager(*mut llvm::LLVMPassManager);

impl Drop for PassManager {
    fn drop(&mut self) {
        unsafe { LLVMDisposePassManager(self.0) };
    }
}

impl PassManager {
    pub fn new(module: &Module) -> Self {
        let prov = module.create_module_provider();

        let res = PassManager(unsafe { LLVMCreateFunctionPassManager(prov) });
        unsafe {
            LLVMAddInstructionCombiningPass(res.0);
            LLVMAddReassociatePass(res.0);
            LLVMAddGVNPass(res.0);
            LLVMAddCFGSimplificationPass(res.0);
            LLVMAddBasicAliasAnalysisPass(res.0);
            LLVMAddPromoteMemoryToRegisterPass(res.0);
            LLVMAddInstructionCombiningPass(res.0);
            LLVMAddReassociatePass(res.0);
            LLVMInitializeFunctionPassManager(res.0);
        }
        res
    }

    pub fn run(&self, fun: &Function) {
        unsafe {
            LLVMRunFunctionPassManager(self.0, fun.0);
        }
    }
}
