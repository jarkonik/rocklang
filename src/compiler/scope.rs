use crate::llvm::{Builder, Context, Module};
use std::collections::HashMap;

use super::{variable::Variable, CompilerResult, Value};

pub struct Scope {
    env: HashMap<String, Variable>,
    params: HashMap<String, Value>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            params: HashMap::new(),
            env: HashMap::new(),
        }
    }

    pub fn get(&self, literal: &str) -> Option<&Variable> {
        self.env.get(literal)
    }

    pub fn set(&mut self, literal: &str, val: Variable) {
        if self.env.contains_key(literal) {
            panic!()
        }
        self.env.insert(literal.to_string(), val);
    }

    pub fn release_references(
        &self,
        context: &Context,
        module: &Module,
        builder: &Builder,
    ) -> CompilerResult<()> {
        for (_, var) in self.env.iter() {
            match var {
                Variable::String(val) => {
                    let release = module.get_function("release_string_reference").unwrap();
                    builder.build_call(
                        &release,
                        &[builder.build_load(&var.llvm_type(context), val, "")],
                        "",
                    );
                }
                Variable::Vec(val) => {
                    let release = module.get_function("release_vec_reference").unwrap();
                    builder.build_call(
                        &release,
                        &[builder.build_load(&var.llvm_type(context), val, "")],
                        "",
                    );
                }
                Variable::I32(_)
                | Variable::F64(_)
                | Variable::Bool(_)
                | Variable::Function { .. }
                | Variable::Ptr(_) => {}
            }
        }
        for (_, var) in self.params.iter() {
            match var {
                Value::String(val) => {
                    let release = module.get_function("release_string_reference").unwrap();
                    builder.build_call(&release, &[*val], "");
                }
                Value::Vec(val) => {
                    let release = module.get_function("release_vec_reference").unwrap();
                    builder.build_call(&release, &[*val], "");
                }
                Value::I32(_)
                | Value::F64(_)
                | Value::Bool(_)
                | Value::Function { .. }
                | Value::Ptr(_) => {}
                Value::Void => unreachable!(),
                Value::Break => unreachable!(),
                Value::CString(_) => todo!(),
            }
        }
        Ok(())
    }

    pub fn set_param(&mut self, name: &str, val: Value) {
        if self.env.contains_key(name) {
            panic!()
        }
        self.params.insert(name.to_string(), val);
    }

    pub fn get_param(&self, name: &str) -> Option<&Value> {
        self.params.get(name)
    }
}
