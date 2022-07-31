use crate::visitor::AssignmentVisitor;

use super::{Compiler, CompilerResult, Value};

impl AssignmentVisitor<CompilerResult<Value>> for Compiler {
    fn visit_assignment(&mut self, expr: &crate::expression::Assignment) -> CompilerResult<Value> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::compiler::scope::Scope;
    use crate::compiler::MAIN_FUNCTION;
    use crate::expression::{self, Assignment};

    #[test]
    fn test_assignment() -> CompilerResult<()> {
        let mut compiler = Compiler::default();
        compiler.scopes.push(Scope::new());

        let expr = Assignment {
            left: Box::new(expression::Expression::Identifier("a".to_string())),
            right: Box::new(expression::Expression::Numeric(10.0)),
        };

        in_main_function!(compiler, {
            let val = compiler.visit_assignment(&expr)?;
            assert!(matches!(val, Value::Null));
        });

        assert_eq!(
            &compiler.ir_string(),
            r#"
        "#
        );

        Ok(())
    }
}
