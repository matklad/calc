use crate::{BinaryOperation, Expr, UnaryOperation};

#[derive(Debug)]
pub enum EvalError {
    DivisionByZero,
}

impl Expr {
    pub fn eval(self: &Expr) -> Result<i64, EvalError> {
        let res = match self {
            Expr::Number(it) => *it,
            Expr::Binary { op, lhs, rhs } => {
                let lhs = lhs.eval()?;
                let rhs = rhs.eval()?;
                match op {
                    BinaryOperation::Addition => lhs + rhs,
                    BinaryOperation::Substraction => lhs - rhs,
                    BinaryOperation::Multiplication => lhs * rhs,
                    BinaryOperation::Division => {
                        lhs.checked_div(rhs).ok_or(EvalError::DivisionByZero)?
                    }
                }
            }
            Expr::Unary {
                op: UnaryOperation::Square,
                arg,
            } => {
                let arg = arg.eval()?;
                arg * arg
            }
        };
        Ok(res)
    }
}
