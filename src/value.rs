use crate::expression::Expression;
use core::fmt;
use core::fmt::Display;

#[derive(Clone, Debug)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Vec<Expression>,
}

#[derive(Clone, Debug)]
pub enum Value {
    Numeric(f64),
    String(String),
    Boolean(bool),
    Function(Function),
    Array(Vec<Value>),
    Null,
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Numeric(v) => write!(f, "{}", v),
            Value::String(v) => write!(f, "{}", v),
            Value::Boolean(v) => write!(f, "{}", v),
            Value::Function(_v) => write!(f, "Function"),
            Value::Array(v) => {
                write!(f, "[")?;
                for (i, x) in v.iter().enumerate() {
                    write!(f, "{}", x)?;
                    if i != v.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")?;
                Ok(())
            }
            Value::Null => write!(f, "null"),
        }
    }
}
