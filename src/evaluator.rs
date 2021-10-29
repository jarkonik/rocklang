use crate::expression;
use crate::expression::Expression;
use crate::expression::Operator;
use crate::parser::Program;
use crate::value::{Function, Value};
use crate::visitor::Visitor;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

pub trait Evaluate {
    fn evaluate(&mut self);
}

pub struct Evaluator {
    program: Program,
    locals: Vec<HashMap<String, Value>>,
}

impl Evaluate for Evaluator {
    fn evaluate(&mut self) {
        self.visit_program(self.program.clone());
    }
}

impl Visitor<Value> for Evaluator {
    fn visit_assignment(&mut self, expr: &expression::Assignment) -> Value {
        match &*expr.left {
            Expression::Identifier(literal) => {
                let val = self.walk(&expr.right);
                self.set_var(literal, val.clone());
                val
            }
            _ => panic!("Evaluation error"),
        }
    }

    fn visit_conditional(&mut self, expr: &expression::Conditional) -> Value {
        let val = self.walk(&expr.predicate);

        match val {
            Value::Boolean(b) => {
                let mut res = Value::Null;
                if b {
                    for stmt in &expr.body {
                        res = self.walk(stmt);
                    }
                } else {
                    for stmt in &expr.else_body {
                        res = self.walk(stmt);
                    }
                }
                res
            }
            _ => panic!("not a boolean"),
        }
    }

    fn visit_binary(&mut self, expr: &expression::Binary) -> Value {
        let l = match self.walk(&expr.left) {
            Value::Numeric(v) => v,
            _ => panic!("not a number"),
        };

        let r = match self.walk(&expr.right) {
            Value::Numeric(v) => v,
            _ => panic!("not a number"),
        };

        match expr.operator {
            Operator::Mod => Value::Numeric(l % r),
            Operator::Plus => Value::Numeric(l + r),
            Operator::Minus => Value::Numeric(l - r),
            Operator::Asterisk => Value::Numeric(l * r),
            Operator::Slash => Value::Numeric(l / r),
            Operator::Equal => Value::Boolean((l - r).abs() < f64::EPSILON),
            Operator::NotEqual => Value::Boolean((l - r).abs() > f64::EPSILON),
            Operator::Less => Value::Boolean(l < r),
            Operator::Greater => Value::Boolean(l > r),
            Operator::LessOrEqual => Value::Boolean(l <= r),
        }
    }

    fn visit_numeric(&mut self, val: &f64) -> Value {
        Value::Numeric(*val)
    }

    fn visit_unary(&mut self, expr: &expression::Unary) -> Value {
        let r = match self.walk(&expr.right) {
            Value::Numeric(v) => v,
            _ => panic!("not a number"),
        };

        let res = match expr.operator {
            Operator::Minus => -r,
            _ => unreachable!(),
        };
        Value::Numeric(res)
    }

    fn visit_grouping(&mut self, expr: &expression::Expression) -> Value {
        self.walk(expr)
    }

    fn visit_func_call(&mut self, expr: &expression::FuncCall) -> Value {
        match &*expr.calee {
            Expression::Identifier(literal) => match literal.as_str() {
                "print" => {
                    if expr.args.len() != 1 {
                        panic!("arity 1 expected");
                    }
                    let res = self.walk(&expr.args[0]);
                    print!("{}", res.to_string().replace("\\n", "\n"));
                    res
                }
                "readln" => {
                    let mut line = String::new();
                    let stdin = io::stdin();
                    stdin.lock().read_line(&mut line).unwrap();
                    line.pop();
                    line.pop();
                    Value::String(line)
                }
                "string" => {
                    if expr.args.len() != 1 {
                        panic!("arity 1 expected");
                    }
                    match self.walk(&expr.args[0]) {
                        Value::String(s) => Value::Numeric(s.parse::<f64>().unwrap()),
                        _ => panic!("not a string"),
                    }
                }
                "arrnew" => {
                    if !expr.args.is_empty() {
                        panic!("arity 0 expected");
                    }

                    Value::Array(vec![])
                }
                "arrget" => {
                    if expr.args.len() != 2 {
                        panic!("arity 2 expected");
                    }

                    let arr = match self.walk(&expr.args[0]) {
                        Value::Array(a) => a,
                        _ => panic!("not an array"),
                    };

                    let idx = match self.walk(&expr.args[1]) {
                        Value::Numeric(n) => n,
                        _ => panic!("not a numeric"),
                    };

                    arr[idx as usize].clone()
                }
                "arrset" => {
                    if expr.args.len() != 3 {
                        panic!("arity 3 expected");
                    }

                    let mut arr = match self.walk(&expr.args[0]) {
                        Value::Array(a) => a,
                        _ => panic!("not an array"),
                    };

                    let idx = match self.walk(&expr.args[1]) {
                        Value::Numeric(n) => n,
                        _ => panic!("not a numeric"),
                    };

                    let val = self.walk(&expr.args[2]);

                    while arr.len() < (idx + 1.0) as usize {
                        arr.push(Value::Null);
                    }

                    arr[idx as usize] = val;

                    Value::Array(arr)
                }
                "timems" => {
                    if !expr.args.is_empty() {
                        panic!("arity 0 expected");
                    }

                    let val = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_millis();
                    Value::Numeric(val as f64)
                }
                "concatstr" => {
                    if expr.args.len() != 2 {
                        panic!("arity 2 expected");
                    }
                    let a = self.walk(&expr.args[0]);
                    let b = self.walk(&expr.args[1]);
                    match (a, b) {
                        (Value::String(a), Value::String(b)) => Value::String(a + &b),
                        _ => panic!("arguments not strings"),
                    }
                }
                _ => {
                    let var = &self.get_var(literal);

                    let mut res = Value::Null;
                    match var {
                        Value::Function(f) => {
                            let mut frame: HashMap<String, Value> = HashMap::new();
                            for i in 0..f.params.len() {
                                frame.insert(f.params[i].to_string(), self.walk(&expr.args[i]));
                            }

                            self.locals.push(frame);

                            for stmt in &f.body {
                                res = self.walk(stmt);
                            }

                            self.locals.pop();

                            res
                        }
                        _ => panic!("not a function"),
                    }
                }
            },
            _ => panic!("Evaluation error"),
        }
    }

    fn visit_while(&mut self, expr: &expression::While) -> Value {
        let mut val = self.walk(&expr.predicate);

        while match val {
            Value::Boolean(v) => v,
            _ => panic!("not a boolean"),
        } {
            let mut break_encountered = false;
            for stmt in &expr.body {
                if matches!(stmt, Expression::Break) {
                    break_encountered = true;
                    break;
                }
                self.walk(stmt);
            }
            if break_encountered {
                break;
            }
            val = self.walk(&expr.predicate);
        }
        Value::Null
    }

    fn visit_identifier(&mut self, expr: &str) -> Value {
        match self.get_var(expr) {
            Value::String(s) => Value::String(s),
            Value::Numeric(v) => Value::Numeric(v),
            Value::Boolean(v) => Value::Boolean(v),
            Value::Function(v) => Value::Function(v),
            Value::Array(v) => Value::Array(v),
            Value::Null => Value::Null,
        }
    }

    fn visit_string(&mut self, expr: &str) -> Value {
        Value::String(expr.to_string())
    }

    fn visit_bool(&mut self, expr: &bool) -> Value {
        Value::Boolean(*expr)
    }

    fn visit_break(&mut self) -> Value {
        Value::Null
    }

    fn visit_program(&mut self, program: Program) -> Value {
        for stmt in program.body {
            self.walk(&stmt);
        }
        Value::Null
    }

    fn visit_func_decl(&mut self, f: &expression::FuncDecl) -> Value {
        Value::Function(Function {
            body: f.body.clone(),
            params: f.params.clone(),
        })
    }
}

impl Evaluator {
    pub fn new(program: Program) -> Self {
        let locals = vec![HashMap::new()];
        Evaluator { program, locals }
    }

    fn set_var(&mut self, name: &str, val: Value) {
        self.locals
            .last_mut()
            .unwrap()
            .insert(name.to_string(), val);
    }

    fn get_var(&mut self, name: &str) -> Value {
        for i in (0..self.locals.len()).rev() {
            if let Some(v) = self.locals[i].get(name) {
                return v.clone();
            }
        }
        panic!("undefined variable {}", name);
    }
}
