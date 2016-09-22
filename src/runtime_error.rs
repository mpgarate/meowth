use std::error;
use std::fmt;
use ast::Expr;

#[derive(Debug)]
pub enum RuntimeError {
  SteppingOnValue(Expr),
  UnexpectedExpr(String, Expr),
  VariableNotFound(String),
}

impl fmt::Display for RuntimeError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      RuntimeError::SteppingOnValue(ref e) => write!(f, "Stepping on a value {:?}", e),
      RuntimeError::UnexpectedExpr(ref s, ref e) => write!(f, "Expected {} and found {:?}", s, e),
      RuntimeError::VariableNotFound(ref e) => write!(f, "Variable {:?} does not exist in memory", e),
    }
  }
}

impl error::Error for RuntimeError {
  fn description(&self) -> &str {
    match *self {
      RuntimeError::SteppingOnValue(_) => "Stepping on a value",
      RuntimeError::UnexpectedExpr(_, _) => "Unexpected expression",
      RuntimeError::VariableNotFound(_) => "Variable does not exist in memory",
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    match *self {
      RuntimeError::SteppingOnValue(_) => None,
      RuntimeError::UnexpectedExpr(_, _) => None,
      RuntimeError::VariableNotFound(_) => None,
    }
  }
}

