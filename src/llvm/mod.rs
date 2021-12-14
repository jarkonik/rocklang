extern crate llvm_sys as llvm;

use core::fmt::Display;
use llvm_sys::analysis::*;
use llvm_sys::transforms::util::LLVMAddPromoteMemoryToRegisterPass;
use std::borrow::Cow;
use std::error::Error;
use std::ffi::CStr;
use std::ffi::CString;
use std::mem;

use llvm::core::*;
use llvm::execution_engine::*;
use llvm::target::*;
use llvm::transforms::scalar::*;
use std::convert::TryInto;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct LLVMError {}

impl fmt::Display for LLVMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LLVMError")
    }
}

impl Error for LLVMError {}

pub(crate) fn c_str(mut s: &str) -> Cow<CStr> {
    if s.is_empty() {
        s = "\0";
    }

    // Start from the end of the string as it's the most likely place to find a null byte
    if !s.chars().rev().any(|ch| ch == '\0') {
        return Cow::from(CString::new(s).expect("unreachable since null bytes are checked"));
    }

    unsafe { Cow::from(CStr::from_ptr(s.as_ptr() as *const _)) }
}

pub struct Engine(*mut llvm::execution_engine::LLVMOpaqueExecutionEngine);

impl Drop for Engine {
    fn drop(&mut self) {
        unsafe { LLVMDisposeExecutionEngine(self.0) }
    }
}

impl Engine {
    #[allow(clippy::uninit_assumed_init)]
    pub fn new(module: &Module) -> Self {
        let mut ee;
        unsafe {
            ee = mem::MaybeUninit::uninit().assume_init();
            let mut out = mem::zeroed();

            LLVMLinkInMCJIT();
            LLVM_InitializeNativeTarget();
            LLVM_InitializeNativeAsmPrinter();

            LLVMCreateExecutionEngineForModule(&mut ee, module.0, &mut out);
        }

        Engine(ee)
    }

    pub fn add_module(&self, module: &Module) {
        unsafe { LLVMAddModule(self.0, module.0) };
    }

    pub fn call(&self, function: Value) -> String {
        let mut params = [];
        unsafe {
            let res = LLVMRunFunction(self.0, function.0, 0, params.as_mut_ptr());
            let ptr = CStr::from_ptr(LLVMGenericValueToPointer(res) as *const i8);

            ptr.to_str().unwrap().to_string()
        }
    }
}

pub struct Builder(*mut llvm::LLVMBuilder);

impl Drop for Builder {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.0);
        }
    }
}

pub enum Cmp {
    Less,
    LessOrEqual,
    Equal,
    NotEqual,
    Greater,
    GreaterOrEqual,
}

impl Builder {
    pub fn new(context: &Context) -> Self {
        Builder(unsafe { LLVMCreateBuilderInContext(context.0) })
    }

    pub fn build_cond_br(&self, iff: &Value, then: &BasicBlock, els: &BasicBlock) -> Value {
        Value(unsafe { LLVMBuildCondBr(self.0, iff.0, then.0, els.0) })
    }

    pub fn build_alloca(&self, el_type: Type, name: &str) -> Value {
        Value(unsafe { LLVMBuildAlloca(self.0, el_type.0, c_str(name).as_ptr()) })
    }

    pub fn create_store(&self, val: Value, ptr: &Value) -> Value {
        Value(unsafe { LLVMBuildStore(self.0, val.0, ptr.0) })
    }

    pub fn build_malloc(&self, el_type: Type, name: &str) -> Value {
        Value(unsafe { LLVMBuildMalloc(self.0, el_type.0, c_str(name).as_ptr()) })
    }

    pub fn build_load(&self, ptr: &Value, name: &str) -> Value {
        Value(unsafe { LLVMBuildLoad(self.0, ptr.0, c_str(name).as_ptr()) })
    }

    pub fn build_add(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        Value(unsafe { LLVMBuildAdd(self.0, lhs.0, rhs.0, c_str(name).as_ptr()) })
    }

    pub fn build_fadd(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        Value(unsafe { LLVMBuildFAdd(self.0, lhs.0, rhs.0, c_str(name).as_ptr()) })
    }

    pub fn build_fsub(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        Value(unsafe { LLVMBuildFSub(self.0, lhs.0, rhs.0, c_str(name).as_ptr()) })
    }

    pub fn build_fdiv(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        Value(unsafe { LLVMBuildFDiv(self.0, lhs.0, rhs.0, c_str(name).as_ptr()) })
    }

    pub fn build_fmul(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        Value(unsafe { LLVMBuildFMul(self.0, lhs.0, rhs.0, c_str(name).as_ptr()) })
    }

    pub fn build_fneg(&self, rhs: Value, name: &str) -> Value {
        Value(unsafe { LLVMBuildFNeg(self.0, rhs.0, c_str(name).as_ptr()) })
    }

    pub fn build_fcmp(&self, lhs: Value, rhs: Value, operator: Cmp, name: &str) -> Value {
        Value(unsafe {
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
        Value(unsafe { LLVMBuildFree(self.0, value.0) })
    }

    pub fn create_br(&self, basic_block: &BasicBlock) -> Value {
        Value(unsafe { LLVMBuildBr(self.0, basic_block.0) })
    }

    pub fn position_builder_at_end(&self, block: &BasicBlock) {
        unsafe { LLVMPositionBuilderAtEnd(self.0, block.0) }
    }

    pub fn build_ret(&self, value: Value) -> Value {
        Value(unsafe { LLVMBuildRet(self.0, value.0) })
    }

    pub fn build_ret_void(&self) -> Value {
        Value(unsafe { LLVMBuildRetVoid(self.0) })
    }

    pub fn get_insert_block(&self) -> BasicBlock {
        BasicBlock(unsafe { LLVMGetInsertBlock(self.0) })
    }

    pub fn build_bitcast(&self, value: &Value, dest_type: Type, name: &str) -> Value {
        Value(unsafe { LLVMBuildBitCast(self.0, value.0, dest_type.0, c_str(name).as_ptr()) })
    }

    pub fn build_call(&self, func: &Value, args: &[Value], name: &str) -> Value {
        let mut args: Vec<*mut llvm::LLVMValue> = args.iter().map(|t| t.0).collect();

        Value(unsafe {
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
        Value(unsafe {
            LLVMBuildGlobalStringPtr(self.0, c_str(str).as_ptr(), c_str(name).as_ptr())
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Value(*mut llvm::LLVMValue);

impl Value {
    pub fn get_param(&self, idx: u32) -> Value {
        Value(unsafe { LLVMGetParam(self.0, idx) })
    }

    pub fn verify_function(&self) -> Result<(), Box<dyn Error>> {
        let result = unsafe {
            LLVMVerifyFunction(self.0, LLVMVerifierFailureAction::LLVMPrintMessageAction)
        };
        if result == 0 {
            Ok(())
        } else {
            Err(Box::new(LLVMError {}))
        }
    }

    pub fn set_initializer(&self, value: Value) {
        unsafe {
            LLVMSetInitializer(self.0, value.0);
        }
    }
}

#[derive(Clone, Copy)]
pub struct Module(*mut llvm::LLVMModule);

impl Module {
    pub fn new(name: &str, context: &Context) -> Self {
        unsafe {
            Module(LLVMModuleCreateWithNameInContext(
                c_str(name).as_ptr(),
                context.0,
            ))
        }
    }

    pub fn get_named_global(&self, name: &str) -> Value {
        Value(unsafe { LLVMGetNamedGlobal(self.0, c_str(name).as_ptr()) })
    }

    pub fn add_global(&self, typ: Type, name: &str) -> Value {
        Value(unsafe { LLVMAddGlobal(self.0, typ.0, c_str(name).as_ptr()) })
    }

    pub fn add_function(&self, name: &str, function_type: Type) -> Value {
        Value(unsafe { LLVMAddFunction(self.0, c_str(name).as_ptr(), function_type.0) })
    }

    pub fn get_function(&self, name: &str) -> Option<Value> {
        let fun = unsafe { LLVMGetNamedFunction(self.0, c_str(name).as_ptr()) };
        if fun.is_null() {
            None
        } else {
            Some(Value(fun))
        }
    }

    pub fn dump(&self) {
        unsafe { LLVMDumpModule(self.0) }
    }
}

impl Display for Module {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let str = unsafe { CStr::from_ptr(LLVMPrintModuleToString(self.0)).to_str() };
        fmt.write_str(str.unwrap())?;
        Ok(())
    }
}

pub struct Context(*mut llvm::LLVMContext);

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            LLVMContextDispose(self.0);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Type(*mut llvm::LLVMType);

impl Type {
    pub fn pointer_type(&self, address_space: u32) -> Type {
        Type(unsafe { LLVMPointerType(self.0, address_space) })
    }
}

pub struct PassManager(*mut llvm::LLVMPassManager);
impl PassManager {
    pub fn new(module: &Module) -> Self {
        let prov = unsafe { LLVMCreateModuleProviderForExistingModule(module.0) };

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

    pub fn run(&self, fun: &Value) {
        unsafe {
            LLVMRunFunctionPassManager(self.0, fun.0);
        }
    }
}

impl PassManager {}

pub struct BasicBlock(*mut llvm::LLVMBasicBlock);

impl BasicBlock {
    pub fn get_parent(&self) -> Value {
        Value(unsafe { LLVMGetBasicBlockParent(self.0) })
    }
}

impl Context {
    pub fn new() -> Self {
        Context(unsafe { LLVMContextCreate() })
    }

    pub fn i64_type(&self) -> Type {
        Type(unsafe { LLVMInt64TypeInContext(self.0) })
    }

    pub fn const_u64_to_ptr(&self, val: Value, tp: Type) -> Value {
        Value(unsafe { LLVMConstIntToPtr(val.0, tp.0) })
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
        BasicBlock(unsafe { LLVMCreateBasicBlockInContext(self.0, c_str(name).as_ptr()) })
    }

    pub fn append_basic_block(&self, function: &Value, name: &str) -> BasicBlock {
        BasicBlock(unsafe {
            LLVMAppendBasicBlockInContext(self.0, function.0, c_str(name).as_ptr())
        })
    }

    pub fn const_float(&self, value: f32) -> Value {
        Value(unsafe { LLVMConstReal(self.float_type().0, value.into()) })
    }

    pub fn const_double(&self, value: f64) -> Value {
        Value(unsafe { LLVMConstReal(self.double_type().0, value) })
    }

    pub fn const_i32(&self, value: i32) -> Value {
        Value(unsafe { LLVMConstInt(self.i32_type().0, value as u64, 0) })
    }

    pub fn const_i8(&self, value: i8) -> Value {
        Value(unsafe { LLVMConstInt(self.i8_type().0, value as u64, 0) })
    }

    pub fn const_u64(&self, value: u64) -> Value {
        Value(unsafe { LLVMConstInt(self.u64_type().0, value, 1) })
    }

    pub fn const_bool(&self, value: bool) -> Value {
        Value(unsafe { LLVMConstInt(self.i1_type().0, if value { 1 } else { 0 }, 0) })
    }
}

impl Default for Context {
    fn default() -> Self {
        Context::new()
    }
}

pub struct FunctionType(*mut llvm::LLVMType);
