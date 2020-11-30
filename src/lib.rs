mod eval;
mod parse;
#[cfg(test)]
mod tests;

use std::fmt;

pub use crate::{eval::EvalError, parse::ParseError};

#[derive(Debug)]
pub enum BinaryOperation {
    Addition,
    Substraction,
    Multiplication,
    Division,
}

#[derive(Debug)]
pub enum UnaryOperation {
    Square,
}

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Binary {
        op: BinaryOperation,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Unary {
        op: UnaryOperation,
        arg: Box<Expr>,
    },
}

pub enum ParseOrEvalError {
    ParseError(ParseError),
    EvalError(EvalError),
}

impl From<EvalError> for ParseOrEvalError {
    fn from(v: EvalError) -> Self {
        ParseOrEvalError::EvalError(v)
    }
}

impl From<ParseError> for ParseOrEvalError {
    fn from(v: ParseError) -> Self {
        ParseOrEvalError::ParseError(v)
    }
}

impl fmt::Display for ParseOrEvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseOrEvalError::ParseError(_) => write!(f, "failed to parse"),
            ParseOrEvalError::EvalError(EvalError::DivisionByZero) => write!(f, "division by zero"),
        }
    }
}

pub fn eval_str(expr: &str) -> Result<i64, ParseOrEvalError> {
    let expr: Expr = expr.parse()?;
    let value = expr.eval()?;
    Ok(value)
}
