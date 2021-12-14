use crate::compiler::Value;
use crate::llvm;
use crate::parser;

#[derive(Debug, Clone, Copy)]
pub enum Var {
    Numeric(llvm::Value),
    Null,
    String(llvm::Value),
    Vec(llvm::Value),
    GlobalString(llvm::Value),
    Bool(llvm::Value),
    Function {
        val: llvm::Value,
        typ: llvm::Type,
        return_type: parser::Type,
    },
}

impl Var {
    pub fn dealloc(&self, context: &llvm::Context, builder: &llvm::Builder) {
        if let Var::Vec(v) = self {
            let fun_type = context.function_type(
                context.void_type(),
                &[context.double_type().pointer_type(0)],
                false,
            );

            let fun_addr = stdlib::vecfree as usize;
            let ptr = context.const_u64_to_ptr(
                context.const_u64(fun_addr.try_into().unwrap()),
                fun_type.pointer_type(0),
            );
            builder.build_call(&ptr, &[builder.build_load(v, "")], "");
        }
    }

    pub fn load(&self, builder: &llvm::Builder) -> Value {
        let val = match self {
            Var::Numeric(p)
            | Var::String(p)
            | Var::GlobalString(p)
            | Var::Vec(p)
            | Var::Bool(p) => builder.build_load(&p, ""),
            Var::Function { val: p, .. } => *p,
            _ => todo!(),
        };

        match self {
            Var::Numeric(_) => Value::Numeric(val),
            Var::String(_) => Value::String(val),
            Var::GlobalString(_) => Value::GlobalString(val),
            Var::Vec(_) => Value::Vec(val),
            Var::Bool(_) => Value::Bool(val),
            Var::Function {
                val,
                return_type,
                typ,
            } => Value::Function {
                val: *val,
                return_type: *return_type,
                typ: *typ,
            },
            _ => todo!(),
        }
    }
}
