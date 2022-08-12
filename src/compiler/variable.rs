use crate::{llvm, parser};

#[derive(Debug, Clone, Copy)]
pub enum Variable {
    String(llvm::Value),
    Numeric(llvm::Value),
    Bool(llvm::Value),
    Function {
        val: llvm::Function,
        typ: llvm::Type,
        return_type: parser::Type,
    },
    Vec(llvm::Value),
    Ptr(llvm::Value),
}

impl From<Variable> for llvm::Value {
    fn from(v: Variable) -> Self {
        match v {
            Variable::String(lv) => lv,
            Variable::Numeric(lv) => lv,
            Variable::Bool(lv) => lv,
            Variable::Function { val, .. } => llvm::Value(val.0),
            Variable::Vec(lv) => lv,
            Variable::Ptr(lv) => lv,
        }
    }
}

impl From<&Variable> for llvm::Value {
    fn from(v: &Variable) -> Self {
        match *v {
            Variable::String(lv) => lv,
            Variable::Numeric(lv) => lv,
            Variable::Bool(lv) => lv,
            Variable::Function { val, .. } => llvm::Value(val.0),
            Variable::Vec(lv) => lv,
            Variable::Ptr(lv) => lv,
        }
    }
}
impl Variable {
    pub fn set_value(&mut self, ptr: llvm::Value) {
        match self {
            Variable::String(v) => {
                v.0 = ptr.0;
            }
            Variable::Numeric(v) => {
                v.0 = ptr.0;
            }
            Variable::Bool(v) => {
                v.0 = ptr.0;
            }
            Variable::Function { val, .. } => {
                val.0 = ptr.0;
            }
            Variable::Vec(v) => {
                v.0 = ptr.0;
            }
            Variable::Ptr(v) => {
                v.0 = ptr.0;
            }
        }
    }
}
