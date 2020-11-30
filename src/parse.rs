use std::{num::ParseIntError, str::FromStr};

use crate::{BinaryOperation, Expr, UnaryOperation};

#[derive(Debug)]
pub enum ParseError {
    ParseIntError(ParseIntError),
    EmptyStack,
    LeftoverStack,
}

impl From<ParseIntError> for ParseError {
    fn from(v: ParseIntError) -> Self {
        ParseError::ParseIntError(v)
    }
}

impl FromStr for Expr {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Expr, ParseError> {
        let mut stack = Vec::new();
        macro_rules! pop {
            () => {
                stack.pop().ok_or(ParseError::EmptyStack)?
            };
        }
        for word in input.split_ascii_whitespace() {
            let expr = match word {
                "sqr" => {
                    let arg = pop!();
                    Expr::Unary {
                        op: UnaryOperation::Square,
                        arg: Box::new(arg),
                    }
                }
                "-" | "+" | "/" | "*" => {
                    let rhs = pop!();
                    let lhs = pop!();
                    let op = match word {
                        "-" => BinaryOperation::Substraction,
                        "+" => BinaryOperation::Addition,
                        "*" => BinaryOperation::Multiplication,
                        "/" => BinaryOperation::Division,
                        _ => unreachable!(),
                    };
                    Expr::Binary {
                        op,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    }
                }
                _ => {
                    let n = word.parse()?;
                    Expr::Number(n)
                }
            };
            stack.push(expr)
        }

        let res = pop!();
        if !stack.is_empty() {
            return Err(ParseError::LeftoverStack);
        }
        Ok(res)
    }
}
