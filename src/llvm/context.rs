extern crate llvm_sys as llvm;

use std::ffi::c_void;

use llvm::core::*;
use llvm::support::LLVMAddSymbol;
use llvm::support::LLVMLoadLibraryPermanently;

use super::utils::c_str;
use super::BasicBlock;
use super::Builder;
use super::Function;
use super::LLVMError;
use super::Module;
use super::Type;
use super::Value;

pub struct Context(*mut llvm::LLVMContext);

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            LLVMContextDispose(self.0);
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Context::new()
    }
}

#[allow(dead_code)]
impl Context {
    pub fn new() -> Self {
        Context(unsafe { LLVMContextCreate() })
    }

    pub fn i64_type(&self) -> Type {
        Type::new(unsafe { LLVMInt64TypeInContext(self.0) })
    }

    pub fn const_u64_to_ptr(&self, val: Value, tp: Type) -> Value {
        Value::from(unsafe { LLVMConstIntToPtr(val.0, tp.0) })
    }

    pub fn function_type(&self, return_type: Type, param_types: &[Type], is_var_arg: bool) -> Type {
        let mut args: Vec<*mut llvm::LLVMType> = param_types.iter().map(|t| t.0).collect();
        Type(unsafe {
            LLVMFunctionType(
                return_type.0,
                args.as_mut_ptr(),
                param_types.len().try_into().unwrap(),
                if is_var_arg { 1 } else { 0 },
            )
        })
    }

    pub fn array_type(&self, el_type: Type, el_count: u32) -> Type {
        Type(unsafe { LLVMArrayType(el_type.0, el_count) })
    }

    pub fn i32_type(&self) -> Type {
        Type(unsafe { LLVMInt32TypeInContext(self.0) })
    }

    pub fn float_type(&self) -> Type {
        Type(unsafe { LLVMFloatTypeInContext(self.0) })
    }

    pub fn double_type(&self) -> Type {
        Type(unsafe { LLVMDoubleTypeInContext(self.0) })
    }

    pub fn void_type(&self) -> Type {
        Type(unsafe { LLVMVoidTypeInContext(self.0) })
    }

    pub fn i8_type(&self) -> Type {
        Type(unsafe { LLVMIntTypeInContext(self.0, 8) })
    }

    pub fn u64_type(&self) -> Type {
        Type(unsafe { LLVMIntTypeInContext(self.0, 64) })
    }

    pub fn i1_type(&self) -> Type {
        Type(unsafe { LLVMIntTypeInContext(self.0, 1) })
    }

    pub fn create_basic_block(&self, name: &str) -> BasicBlock {
        BasicBlock::new(unsafe { LLVMCreateBasicBlockInContext(self.0, c_str(name).as_ptr()) })
    }

    pub fn append_basic_block(&self, function: &Function, name: &str) -> BasicBlock {
        BasicBlock::new(unsafe {
            LLVMAppendBasicBlockInContext(self.0, function.0, c_str(name).as_ptr())
        })
    }

    pub fn const_float(&self, value: f32) -> Value {
        Value::from(unsafe { LLVMConstReal(self.float_type().0, value.into()) })
    }

    pub fn const_double(&self, value: f64) -> Value {
        Value::from(unsafe { LLVMConstReal(self.double_type().0, value) })
    }

    pub fn add_symbol(&self, name: &str, f: *mut c_void) {
        unsafe { LLVMAddSymbol(c_str(name).as_ptr(), f) };
    }

    pub fn const_i8(&self, value: i8) -> Value {
        Value::from(unsafe { LLVMConstInt(self.i8_type().0, value as u64, 0) })
    }

    pub fn const_u64(&self, value: u64) -> Value {
        Value::from(unsafe { LLVMConstInt(self.u64_type().0, value, 1) })
    }

    pub fn const_bool(&self, value: bool) -> Value {
        Value::from(unsafe { LLVMConstInt(self.i1_type().0, if value { 1 } else { 0 }, 0) })
    }

    pub fn load_libary_permanently(&self, name: &str) -> Result<(), LLVMError> {
        unsafe {
            if LLVMLoadLibraryPermanently(c_str(name).as_ptr()) != 0 {
                Err(LLVMError {})?;
            }
        }
        Ok(())
    }

    pub fn create_builder(&self) -> Builder {
        Builder::from(unsafe { LLVMCreateBuilderInContext(self.0) })
    }

    pub fn create_module(&self, name: &str) -> Module {
        Module::from_context(unsafe {
            LLVMModuleCreateWithNameInContext(c_str(name).as_ptr(), self.0)
        })
    }
}
