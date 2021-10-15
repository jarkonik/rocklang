extern crate llvm_sys as llvm;

use std::borrow::Cow;
use std::ffi::CStr;
use std::ffi::CString;
use std::mem;

use llvm::core::*;
use llvm::execution_engine::*;
use llvm::target::*;
use std::convert::TryInto;

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

	pub fn call(&self, name: &str) {
		unsafe {
			let addr = LLVMGetFunctionAddress(self.0, c_str(name).as_ptr());
			let f: extern "C" fn() = mem::transmute(addr);
			f();
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

impl Builder {
	pub fn new(context: &Context) -> Self {
		Builder(unsafe { LLVMCreateBuilderInContext(context.0) })
	}

	pub fn build_alloca(&self, el_type: Type, name: &str) -> Value {
		Value(unsafe { LLVMBuildAlloca(self.0, el_type.0, c_str(name).as_ptr()) })
	}

	pub fn build_malloc(&self, el_type: Type, name: &str) -> Value {
		Value(unsafe { LLVMBuildMalloc(self.0, el_type.0, c_str(name).as_ptr()) })
	}

	pub fn create_br(&self, basic_block: &BasicBlock) -> Value {
		Value(unsafe { LLVMBuildBr(self.0, basic_block.0) })
	}

	pub fn position_builder_at_end(&self, block: &BasicBlock) {
		unsafe { LLVMPositionBuilderAtEnd(self.0, block.0) }
	}

	pub fn build_add(&self, lhs: Value, rhs: Value, name: &str) -> Value {
		Value(unsafe { LLVMBuildAdd(self.0, lhs.0, rhs.0, c_str(name).as_ptr()) })
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

	pub fn build_call(&self, func: Value, args: &[&Value], name: &str) -> Value {
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

#[derive(Debug)]
pub struct Value(*mut llvm::LLVMValue);

impl Value {
	pub fn get_param(&self, idx: u32) -> Value {
		Value(unsafe { LLVMGetParam(self.0, idx) })
	}
}

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

	pub fn add_function(&self, name: &str, function_type: FunctionType) -> Value {
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
		Value(unsafe { LLVMConstReal(self.double_type().0, value.into()) })
	}

	pub fn const_i32(&self, value: i32) -> Value {
		Value(unsafe { LLVMConstInt(self.i32_type().0, value as u64, 0) })
	}
}

impl Default for Context {
	fn default() -> Self {
		Context::new()
	}
}

pub struct FunctionType(*mut llvm::LLVMType);

impl FunctionType {
	pub fn new(return_type: Type, param_types: &[Type], is_var_arg: bool) -> Self {
		let mut args: Vec<*mut llvm::LLVMType> = param_types.iter().map(|t| t.0).collect();

		FunctionType(unsafe {
			LLVMFunctionType(
				return_type.0,
				args.as_mut_ptr(),
				param_types.len().try_into().unwrap(),
				if is_var_arg { 1 } else { 0 },
			)
		})
	}
}
