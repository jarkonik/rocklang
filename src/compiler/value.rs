use crate::llvm::Context;
use crate::llvm::{self};
use crate::parser;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Void,
    String(llvm::Value),
    Numeric(llvm::Value),
    Bool(llvm::Value),
    Function {
        val: llvm::Function,
        typ: llvm::Type,
        return_type: parser::Type,
    },
    Vec(llvm::Value),
    Break,
    Ptr(llvm::Value),
}

impl From<Value> for llvm::Value {
    fn from(v: Value) -> Self {
        match v {
            Value::Void | Value::Break => unimplemented!(),
            Value::String(lv) => lv,
            Value::Numeric(lv) => lv,
            Value::Bool(lv) => lv,
            Value::Function { val, .. } => llvm::Value(val.0),
            Value::Vec(lv) => lv,
            Value::Ptr(lv) => lv,
        }
    }
}

impl From<&Value> for llvm::Value {
    fn from(v: &Value) -> Self {
        match *v {
            Value::Void | Value::Break => unimplemented!(),
            Value::String(lv) => lv,
            Value::Numeric(lv) => lv,
            Value::Bool(lv) => lv,
            Value::Function { val, .. } => llvm::Value(val.0),
            Value::Vec(lv) => lv,
            Value::Ptr(lv) => lv,
        }
    }
}

impl Value {
    pub fn llvm_type(&self, context: &Context) -> llvm::Type {
        match self {
            Value::Void => context.void_type(),
            Value::Numeric(_) => context.double_type(),
            Value::Bool(_) => context.i1_type(),
            Value::Ptr(_) => context.void_type().pointer_type(0),
            Value::String(_) => context.void_type().pointer_type(0),
            Value::Vec(_) => context.void_type().pointer_type(0),
            Value::Function { typ, .. } => typ.pointer_type(0),
            Value::Break => unimplemented!(),
        }
    }
}
