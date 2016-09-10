use std::error;
use std::fmt;
use parser::token::Token;

#[derive(Debug)]
pub enum ParserError {
  UnexpectedToken(Token, Token), // expected, actual
  InvalidToken(Token, String),
  InvalidFactor(String),
}

impl fmt::Display for ParserError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      ParserError::UnexpectedToken(ref t1, ref t2) => write!(f, "expected token: {:?} actual: {:?}", t1, t2),
      ParserError::InvalidToken(ref t, ref s) => write!(f, "invalid token {:?} while {}", t, s),
      ParserError::InvalidFactor(ref s) => write!(f, "{}", s),
    }
  }
}

impl error::Error for ParserError {
  fn description(&self) -> &str {
    match *self {
      ParserError::UnexpectedToken(_, _) => "next token does not match expected",
      ParserError::InvalidToken(_, _) => "token invalid for context",
      ParserError::InvalidFactor(ref s) => s,
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    match *self {
      ParserError::UnexpectedToken(_, _) => None,
      ParserError::InvalidToken(_, _) => None,
      ParserError::InvalidFactor(_) => None,
    }
  }
}

