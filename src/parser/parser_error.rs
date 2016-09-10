use std::error;
use std::fmt;

#[derive(Debug)]
pub enum ParserError {
  UnexpectedToken(String),
  InvalidFactor(String),
}

impl fmt::Display for ParserError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      ParserError::UnexpectedToken(ref s) => write!(f, "{}", s),
      ParserError::InvalidFactor(ref s) => write!(f, "{}", s),
    }
  }
}

impl error::Error for ParserError {
  fn description(&self) -> &str {
    match *self {
      ParserError::UnexpectedToken(ref s) => s,
      ParserError::InvalidFactor(ref s) => s,
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    match *self {
      ParserError::UnexpectedToken(_) => None,
      ParserError::InvalidFactor(_) => None,
    }
  }
}

