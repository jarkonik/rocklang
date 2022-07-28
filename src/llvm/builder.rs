extern crate llvm_sys as llvm;

use llvm::{core::*, LLVMBuilder};

use super::{utils::c_str, BasicBlock, Context, Function, Type, Value};

pub struct Builder(*mut llvm::LLVMBuilder);
pub enum Cmp {
    Less,
    LessOrEqual,
    Equal,
    NotEqual,
    Greater,
    GreaterOrEqual,
}

impl Drop for Builder {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.0);
        }
    }
}

#[allow(dead_code)]
impl Builder {
    pub fn new(context: &Context) -> Self {
        context.create_builder()
    }

    pub fn from(ptr: *mut LLVMBuilder) -> Self {
        Builder(ptr)
    }

    pub fn build_cond_br(&self, iff: &Value, then: &BasicBlock, els: &BasicBlock) -> Value {
        Value::from(unsafe { LLVMBuildCondBr(self.0, iff.0, then.0, els.0) })
    }

    pub fn build_br(&self, dest: &BasicBlock) -> Value {
        Value::from(unsafe { LLVMBuildBr(self.0, dest.0) })
    }

    pub fn build_alloca(&self, el_type: Type, name: &str) -> Value {
        Value::from(unsafe { LLVMBuildAlloca(self.0, el_type.0, c_str(name).as_ptr()) })
    }

    pub fn create_store(&self, val: Value, ptr: &Value) -> Value {
        Value::from(unsafe { LLVMBuildStore(self.0, val.0, ptr.0) })
    }

    pub fn build_malloc(&self, el_type: Type, name: &str) -> Value {
        Value::from(unsafe { LLVMBuildMalloc(self.0, el_type.0, c_str(name).as_ptr()) })
    }

    pub fn build_load(&self, ptr: &Value, name: &str) -> Value {
        Value::from(unsafe { LLVMBuildLoad(self.0, ptr.0, c_str(name).as_ptr()) })
    }

    pub fn build_add(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        Value::from(unsafe { LLVMBuildAdd(self.0, lhs.0, rhs.0, c_str(name).as_ptr()) })
    }

    pub fn build_fadd(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        Value::from(unsafe { LLVMBuildFAdd(self.0, lhs.0, rhs.0, c_str(name).as_ptr()) })
    }

    pub fn build_fsub(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        Value::from(unsafe { LLVMBuildFSub(self.0, lhs.0, rhs.0, c_str(name).as_ptr()) })
    }

    pub fn build_fdiv(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        Value::from(unsafe { LLVMBuildFDiv(self.0, lhs.0, rhs.0, c_str(name).as_ptr()) })
    }

    pub fn build_fmul(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        Value::from(unsafe { LLVMBuildFMul(self.0, lhs.0, rhs.0, c_str(name).as_ptr()) })
    }

    pub fn build_fneg(&self, rhs: Value, name: &str) -> Value {
        Value::from(unsafe { LLVMBuildFNeg(self.0, rhs.0, c_str(name).as_ptr()) })
    }

    pub fn build_fcmp(&self, lhs: Value, rhs: Value, operator: Cmp, name: &str) -> Value {
        Value::from(unsafe {
            LLVMBuildFCmp(
                self.0,
                match operator {
                    Cmp::LessOrEqual => llvm::LLVMRealPredicate::LLVMRealOLE,
                    Cmp::Less => llvm::LLVMRealPredicate::LLVMRealOLT,
                    Cmp::GreaterOrEqual => llvm::LLVMRealPredicate::LLVMRealOGE,
                    Cmp::Greater => llvm::LLVMRealPredicate::LLVMRealOGT,
                    Cmp::Equal => llvm::LLVMRealPredicate::LLVMRealOEQ,
                    Cmp::NotEqual => llvm::LLVMRealPredicate::LLVMRealONE,
                },
                lhs.0,
                rhs.0,
                c_str(name).as_ptr(),
            )
        })
    }

    pub fn build_free(&self, value: Value) -> Value {
        Value::from(unsafe { LLVMBuildFree(self.0, value.0) })
    }

    pub fn create_br(&self, basic_block: &BasicBlock) -> Value {
        Value::from(unsafe { LLVMBuildBr(self.0, basic_block.0) })
    }

    pub fn position_builder_at_end(&self, block: &BasicBlock) {
        unsafe { LLVMPositionBuilderAtEnd(self.0, block.0) }
    }

    pub fn build_ret(&self, value: Value) -> Value {
        Value::from(unsafe { LLVMBuildRet(self.0, value.0) })
    }

    pub fn build_ret_void(&self) -> Value {
        Value::from(unsafe { LLVMBuildRetVoid(self.0) })
    }

    pub fn get_insert_block(&self) -> BasicBlock {
        BasicBlock(unsafe { LLVMGetInsertBlock(self.0) })
    }

    pub fn build_bitcast(&self, value: &Value, dest_type: Type, name: &str) -> Value {
        Value::from(unsafe { LLVMBuildBitCast(self.0, value.0, dest_type.0, c_str(name).as_ptr()) })
    }

    pub fn build_call(&self, func: &Function, args: &[Value], name: &str) -> Value {
        let mut args: Vec<*mut llvm::LLVMValue> = args.iter().map(|t| t.0).collect();

        Value::from(unsafe {
            LLVMBuildCall(
                self.0,
                func.0,
                args.as_mut_ptr(),
                args.len().try_into().unwrap(),
                c_str(name).as_ptr(),
            )
        })
    }

    pub fn build_global_string_ptr(&self, str: &str, name: &str) -> Value {
        Value::from(unsafe {
            LLVMBuildGlobalStringPtr(self.0, c_str(str).as_ptr(), c_str(name).as_ptr())
        })
    }
}
