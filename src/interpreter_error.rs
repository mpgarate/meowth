use std::error;
use std::fmt;
use ast::Expr;

#[derive(Debug)]
pub enum InterpreterError {
  SteppingOnValue(Expr),
  UnexpectedExpr(String, Expr),
  VariableNotFound(String),
}

impl fmt::Display for InterpreterError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      InterpreterError::SteppingOnValue(ref e) => write!(f, "{:?}", e),
      InterpreterError::UnexpectedExpr(ref s, ref e) => write!(f, "{}, got {:?}", s, e),
      InterpreterError::VariableNotFound(ref e) => write!(f, "{:?}", e),
    }
  }
}

impl error::Error for InterpreterError {
  fn description(&self) -> &str {
    match *self {
      InterpreterError::SteppingOnValue(_) => "Stepping on a value",
      InterpreterError::UnexpectedExpr(_, _) => "Unexpected expression",
      InterpreterError::VariableNotFound(_) => "Variable does not exist in memory",
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    match *self {
      InterpreterError::SteppingOnValue(_) => None,
      InterpreterError::UnexpectedExpr(_, _) => None,
      InterpreterError::VariableNotFound(_) => None,
    }
  }
}

