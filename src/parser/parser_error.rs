use std::error;
use std::fmt;
use parser::token::Token;
use parser::lexer_error::LexerError;

#[derive(Debug)]
pub enum ParserError {
  UnexpectedToken(Token, Token), // expected, actual
  InvalidToken(Token, String),
  LexerError(LexerError),
}

impl fmt::Display for ParserError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      ParserError::UnexpectedToken(ref t1, ref t2) => write!(f, "expected token: {:?} actual: {:?}", t1, t2),
      ParserError::InvalidToken(ref t, ref s) => write!(f, "invalid token {:?} while {}", t, s),
      ParserError::LexerError(ref err) => write!(f, "Lexer error: {}", err),
    }
  }
}

impl error::Error for ParserError {
  fn description(&self) -> &str {
    match *self {
      ParserError::UnexpectedToken(_, _) => "next token does not match expected",
      ParserError::InvalidToken(_, _) => "token invalid for context",
      ParserError::LexerError(ref err) => err.description(),
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    match *self {
      ParserError::UnexpectedToken(_, _) => None,
      ParserError::InvalidToken(_, _) => None,
      ParserError::LexerError(ref err) => Some(err),
    }
  }
}

impl From<LexerError> for ParserError {
  fn from(err: LexerError) -> ParserError {
    ParserError::LexerError(err)
  }
}

