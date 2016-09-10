use parser::lexer_error::LexerError;

#[derive(Clone, Debug, PartialEq)] 
pub enum Token {
  Plus,
  Minus,
  Times,
  Div,
  RParen,
  LParen,
  Eq,
  Ne,
  Leq,
  Geq,
  Lt,
  Gt,
  Not,
  And,
  Or,
  Mod,
  Seq,
  Ternary,
  Else,
  Var(String),
  Int(isize),
  Bool(bool),
  Let,
  VarDecl,
  Assign,
  FnDecl,
  LBracket,
  RBracket,
  Comma,
  If,
  While,
  EOF,
}

impl Token {
  pub fn is_term_op(&self) -> bool {
    match *self {
      Token::Times => true,
      Token::Div => true,
      _ => false,
    }
  
  }

  pub fn is_expr_op(&self) -> bool {
    match *self {
      Token::Plus => true,
      Token::Minus => true,
      Token::Eq => true,
      Token::Ne => true,
      Token::Leq => true,
      Token::Geq => true,
      Token::Lt => true,
      Token::Gt => true,
      Token::And => true,
      Token::Or => true,
      Token::Mod => true,
      _ => false,
    }
  }

  pub fn is_statement_op(&self) -> bool {
    match *self {
      Token::Ternary => true,
      Token::Assign => true,
      _ => false,
    }
  }

  pub fn is_block_op(&self) -> bool {
    match *self {
      Token::Seq => true,
      _ => false,
    }
  }
}

pub struct Lexer {
  text: String,
}

impl Lexer {
  pub fn new(text: String) -> Lexer {
    Lexer {
      text: text,
    }
  }

  fn advance(&mut self, n: usize) {
    let text = self.text.clone();
    let (_, t) = text.split_at(n);
    self.text = t.to_string();
  }

  fn lex_integer(&mut self) -> Result<Token, LexerError> {
    let int_str: String = self.text
      .chars()
      .take_while(|c| c.is_digit(10))
      .collect();

    let n = try!(int_str.parse::<isize>());

    self.advance(int_str.len());
    Ok(Token::Int(n))
  }

  fn lex_keyword(&mut self) -> Result<Token, LexerError> {
    let keyword: String = self.text
      .chars()
      .take_while(|c| c.is_alphabetic() || *c == '_')
      .collect();

    self.advance(keyword.len());

    let token = match keyword.as_ref() {
      "true" => Token::Bool(true),
      "false" => Token::Bool(false),
      "fn" => Token::FnDecl,
      "let" => Token::Let,
      "var" => Token::VarDecl,
      "if" => Token::If,
      "else" => Token::Else,
      "while" => Token::While,
      s if s.len() > 0 => Token::Var(s.to_string()),
      s => return Err(LexerError::InvalidKeyword(format!("invalid keyword {:?}", s)))
    };

    Ok(token)
  }

  fn skip_whitespace(&mut self) {
    let spaces_str: String = self.text
      .chars()
      .take_while(|c| c.is_whitespace())
      .collect();

    self.advance(spaces_str.len());
  }

  fn skip_inline_comment(&mut self) {
    let comment_str: String = self.text
      .chars()
      .take_while(|c| !(c == &'\n'))
      .collect();

    self.advance(comment_str.len());
  }

  fn skip_block_comment(&mut self) {
    // TODO: remove everything through the next '*/'
  }

  fn peek_next(&mut self) -> Option<char> {
    self.text.chars().next()
  }

  pub fn get_next_token(&mut self) -> Result<Token, LexerError> {
    while self.peek_next() != None {
      debug!("get_next_token: {}", self.text);

      let token = match self.peek_next() {
        Some('/') if self.text.starts_with("//") => {
          self.advance(2);
          self.skip_inline_comment();
          continue;
        },
        Some('/') if self.text.starts_with("/*") => {
          self.advance(2);
          self.skip_block_comment();
          continue;
        },
        Some('+') => {
          self.advance(1);
          Token::Plus
        },
        Some('-') => {
          self.advance(1);
          Token::Minus
        },
        Some('*') => {
          self.advance(1);
          Token::Times
        },
        Some('/') => {
          self.advance(1);
          Token::Div
        },
        Some('%') => {
          self.advance(1);
          Token::Mod
        },
        Some('(') => {
          self.advance(1);
          Token::LParen
        },
        Some(')') => {
          self.advance(1);
          Token::RParen
        },
        Some('&') if self.text.starts_with("&&") => {
          self.advance(2);
          Token::And
        },
        Some('|') if self.text.starts_with("||") => {
          self.advance(2);
          Token::Or
        },
        Some('=') if self.text.starts_with("==") => {
          self.advance(2);
          Token::Eq
        },
        Some('=') => {
          self.advance(1);
          Token::Assign
        },
        Some('!') if self.text.starts_with("!=") => {
          self.advance(2);
          Token::Ne
        },
        Some('!') => {
          self.advance(1);
          Token::Not
        },
        Some('>') if self.text.starts_with(">=") => {
          self.advance(2);
          Token::Geq
        },
        Some('>') => {
          self.advance(1);
          Token::Gt
        },
        Some('<') if self.text.starts_with("<=") => {
          self.advance(2);
          Token::Leq
        },
        Some('<') => {
          self.advance(1);
          Token::Lt
        },
        Some(';') => {
          self.advance(1);
          Token::Seq
        },
        Some('?') => {
          self.advance(1);
          Token::Ternary
        },
        Some(':') => {
          self.advance(1);
          Token::Else
        },
        Some('{') => {
          self.advance(1);
          Token::LBracket
        },
        Some('}') => {
          self.advance(1);
          Token::RBracket
        },
        Some(',') => {
          self.advance(1);
          Token::Comma
        },
        Some(c) if c.is_alphabetic() => return self.lex_keyword(),
        Some(c) if c.is_digit(10) => return self.lex_integer(),
        Some(c) if c.is_whitespace() => {
          self.skip_whitespace();
          continue;
        },
        c => return Err(LexerError::UnexpectedCharacter(format!("unexpected char {:?}", c))),
      };

      return Ok(token)
    }

    Ok(Token::EOF)
  }
}
