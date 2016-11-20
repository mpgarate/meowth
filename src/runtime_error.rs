use std::error;
use std::fmt;
use expr::Expr;
use parser::parser_error::ParserError;

#[derive(Debug, PartialEq)]
pub enum RuntimeError {
  SteppingOnValue(Expr),
  UnexpectedExpr(String, Expr),
  VariableNotFound(String),
  InvalidConstAssignment(Expr, String),
  InvalidTypeConversion(String, Expr),
  InvalidMemoryState(String),
  ParserError(ParserError),
  TooManyIterations(usize),
  EmptyBike(String),
  GiveFromConst(String),
}

impl fmt::Display for RuntimeError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      RuntimeError::SteppingOnValue(ref e) => write!(f, "Stepping on a value {:?}", e),
      RuntimeError::UnexpectedExpr(ref s, ref e) => write!(f, "Unexpected expression. Expected {} and found {:?}", s, e),
      RuntimeError::VariableNotFound(ref e) => write!(f, "Variable {:?} does not exist in memory", e),
      RuntimeError::InvalidConstAssignment(ref e, ref s) => write!(f, "Cannot assign {:?} to const {}", e, s),
      RuntimeError::InvalidTypeConversion(ref s, ref e) => write!(f, "Invalid type conversion. Expected {} and found {:?}", s, e),
      RuntimeError::InvalidMemoryState(ref s) => write!(f, "Unexpected internal memory state: {}", s),
      RuntimeError::TooManyIterations(ref n) => write!(f, "Too many iterations while evaluating expression: {}", n),
      RuntimeError::EmptyBike(ref s) => write!(f, "No value in empty bike {}", s),
      RuntimeError::GiveFromConst(ref s) => write!(f, "Cannot give from const {}", s),
      RuntimeError::ParserError(ref err) => write!(f, "Parser error: {}", err),
    }
  }
}

impl error::Error for RuntimeError {
  fn description(&self) -> &str {
    match *self {
      RuntimeError::SteppingOnValue(_) => "Stepping on a value",
      RuntimeError::UnexpectedExpr(_, _) => "Unexpected expression",
      RuntimeError::VariableNotFound(_) => "Variable does not exist in memory",
      RuntimeError::InvalidConstAssignment(_, _) => "Cannot assign to const",
      RuntimeError::InvalidTypeConversion(_, _) => "Invalid type conversion",
      RuntimeError::InvalidMemoryState(_) => "Unexpected internal memory state",
      RuntimeError::TooManyIterations(_) => "Too many iterations",
      RuntimeError::EmptyBike(_) => "No value in empty bike",
      RuntimeError::GiveFromConst(_) => "Cannot give from const",
      RuntimeError::ParserError(ref err) => err.description(),
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    match *self {
      RuntimeError::SteppingOnValue(_) => None,
      RuntimeError::UnexpectedExpr(_, _) => None,
      RuntimeError::VariableNotFound(_) => None,
      RuntimeError::InvalidConstAssignment(_, _) => None,
      RuntimeError::InvalidTypeConversion(_, _) => None,
      RuntimeError::InvalidMemoryState(_) => None,
      RuntimeError::TooManyIterations(_) => None,
      RuntimeError::EmptyBike(_) => None,
      RuntimeError::GiveFromConst(_) => None,
      RuntimeError::ParserError(ref err) => Some(err),
    }
  }
}

impl From<ParserError> for RuntimeError {
  fn from(err: ParserError) -> RuntimeError {
    RuntimeError::ParserError(err)
  }
}
