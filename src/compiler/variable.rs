use crate::{
    llvm::{self, Context},
    parser,
};

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
    pub fn llvm_type(&self, context: &Context) -> llvm::Type {
        match self {
            Variable::Numeric(_) => context.double_type(),
            Variable::Bool(_) => context.i1_type(),
            Variable::Ptr(_) => context.void_type().pointer_type(0),
            Variable::String(_) => context.void_type().pointer_type(0),
            Variable::Vec(_) => context.void_type().pointer_type(0),
            Variable::Function { typ, .. } => typ.pointer_type(0),
        }
    }

    pub fn get_type(&self) -> parser::Type {
        match self {
            Variable::Numeric(_) => parser::Type::Numeric,
            Variable::Bool(_) => parser::Type::Bool,
            Variable::Ptr(_) => parser::Type::Ptr,
            Variable::String(_) => parser::Type::String,
            Variable::Vec(_) => parser::Type::Vector,
            Variable::Function { .. } => parser::Type::Function,
        }
    }

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
