use std::num;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum LexerError {
  UnexpectedCharacter(String),
  InvalidKeyword(String),
  ParseInt(num::ParseIntError),
}

impl fmt::Display for LexerError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      LexerError::UnexpectedCharacter(ref s) => write!(f, "{}", s),
      LexerError::InvalidKeyword(ref s) => write!(f, "{}", s),
      LexerError::ParseInt(ref err) => write!(f, "Parse error: {}", err),
    }
  }
}

impl error::Error for LexerError {
  fn description(&self) -> &str {
    match *self {
      LexerError::UnexpectedCharacter(ref s) => s,
      LexerError::InvalidKeyword(ref s) => s,
      LexerError::ParseInt(ref err) => err.description(),
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    match *self {
      LexerError::UnexpectedCharacter(_) => None,
      LexerError::InvalidKeyword(_) => None,
      LexerError::ParseInt(ref err) => Some(err),
    }
  }
}

impl From<num::ParseIntError> for LexerError {
  fn from(err: num::ParseIntError) -> LexerError {
    LexerError::ParseInt(err)
  }
}

