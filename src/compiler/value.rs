use crate::llvm::Context;
use crate::llvm::{self};
use crate::parser;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Void,
    String(llvm::Value),
    CString(llvm::Value),
    F64(llvm::Value),
    Bool(llvm::Value),
    Function {
        val: llvm::Function,
        typ: llvm::Type,
        return_type: parser::Type,
    },
    Vec(llvm::Value),
    Break,
    Ptr(llvm::Value),
    I32(llvm::Value),
}

impl From<Value> for llvm::Value {
    fn from(v: Value) -> Self {
        match v {
            Value::Void | Value::Break => unreachable!(),
            Value::String(lv) => lv,
            Value::F64(lv) => lv,
            Value::Bool(lv) => lv,
            Value::Function { val, .. } => llvm::Value(val.0),
            Value::Vec(lv) => lv,
            Value::Ptr(lv) => lv,
            Value::CString(_) => todo!(),
            Value::I32(lv) => lv,
        }
    }
}

impl From<&Value> for llvm::Value {
    fn from(v: &Value) -> Self {
        match *v {
            Value::Void | Value::Break => unreachable!(),
            Value::String(lv) => lv,
            Value::F64(lv) => lv,
            Value::Bool(lv) => lv,
            Value::Function { val, .. } => llvm::Value(val.0),
            Value::Vec(lv) => lv,
            Value::Ptr(lv) => lv,
            Value::CString(_) => todo!(),
            Value::I32(lv) => lv,
        }
    }
}

impl Value {
    pub fn llvm_type(&self, context: &Context) -> llvm::Type {
        match self {
            Value::F64(_) => context.double_type(),
            Value::Bool(_) => context.i1_type(),
            Value::Ptr(_) => context.void_type().pointer_type(0),
            Value::String(_) => context.void_type().pointer_type(0),
            Value::Vec(_) => context.void_type().pointer_type(0),
            Value::Function { typ, .. } => typ.pointer_type(0),
            Value::Void | Value::Break => unreachable!(),
            Value::CString(_) => todo!(),
            Value::I32(_) => context.i32_type(),
        }
    }

    pub fn get_type(&self) -> parser::Type {
        match self {
            Value::Void | Value::Break => parser::Type::Void,
            Value::F64(_) => parser::Type::F64,
            Value::Bool(_) => parser::Type::Bool,
            Value::Ptr(_) => parser::Type::Ptr,
            Value::String(_) => parser::Type::String,
            Value::Vec(_) => parser::Type::Vector,
            Value::Function { .. } => parser::Type::Function,
            Value::CString(_) => parser::Type::CString,
            Value::I32(_) => parser::Type::I32,
        }
    }
}
