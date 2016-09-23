use std::error;
use std::fmt;
use ast::Expr;
use parser::parser_error::ParserError;

#[derive(Debug)]
pub enum RuntimeError {
  SteppingOnValue(Expr),
  UnexpectedExpr(String, Expr),
  VariableNotFound(String),
  ParserError(ParserError),
}

impl fmt::Display for RuntimeError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      RuntimeError::SteppingOnValue(ref e) => write!(f, "Stepping on a value {:?}", e),
      RuntimeError::UnexpectedExpr(ref s, ref e) => write!(f, "Expected {} and found {:?}", s, e),
      RuntimeError::VariableNotFound(ref e) => write!(f, "Variable {:?} does not exist in memory", e),
      RuntimeError::ParserError(ref err) => write!(f, "Parser error {}", err),
    }
  }
}

impl error::Error for RuntimeError {
  fn description(&self) -> &str {
    match *self {
      RuntimeError::SteppingOnValue(_) => "Stepping on a value",
      RuntimeError::UnexpectedExpr(_, _) => "Unexpected expression",
      RuntimeError::VariableNotFound(_) => "Variable does not exist in memory",
      RuntimeError::ParserError(ref err) => err.description(),
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    match *self {
      RuntimeError::SteppingOnValue(_) => None,
      RuntimeError::UnexpectedExpr(_, _) => None,
      RuntimeError::VariableNotFound(_) => None,
      RuntimeError::ParserError(ref err) => Some(err),
    }
  }
}

impl From<ParserError> for RuntimeError {
  fn from(err: ParserError) -> RuntimeError {
    RuntimeError::ParserError(err)
  }
}
