use std::error;
use std::fmt;
use ast::Expr;

#[derive(Debug)]
pub enum InterpreterError {
  SteppingOnValue(Expr)
}
