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
  Assign,
  FnDecl,
  FnCall(String),
  LBracket,
  RBracket,
  EOF,
}

impl Token {
  pub fn is_term_bop(&self) -> bool {
    match *self {
      Token::Times => true,
      Token::Div => true,
      _ => false,
    }
  
  }

  pub fn is_expr_bop(&self) -> bool {
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

  fn lex_integer(&mut self) -> Token {
    let int_str: String = self.text
      .chars()
      .take_while(|c| c.is_digit(10))
      .collect();

    match int_str.parse::<isize>() {
      Ok(n) => {
        self.advance(int_str.len());
        return Token::Int(n);
      }
      Err(_) => panic!()
    }
  }

  fn lex_keyword(&mut self) -> Token {
    let keyword: String = self.text
      .chars()
      .take_while(|c| c.is_alphabetic())
      .collect();

    self.advance(keyword.len());

    let next_char = self.peek_next();

    match keyword.as_ref()  {
      "true" => Token::Bool(true),
      "false" => Token::Bool(false),
      "fn" => Token::FnDecl,
      "let" => Token::Let,
      s if next_char == Some('(') => Token::FnCall(s.to_string()),
      s if s.len() > 0 => Token::Var(s.to_string()),
      _ => panic!()
    }
  }

  fn skip_whitespace(&mut self) {
    let spaces_str: String = self.text
      .chars()
      .take_while(|c| c.is_whitespace())
      .collect();

    self.advance(spaces_str.len());
  }

  fn peek_next(&mut self) -> Option<char> {
    self.text.chars().next()
  }

  pub fn get_next_token(&mut self) -> Token {
    while self.peek_next() != None {
      debug!("get_next_token: {}", self.text);

      match self.peek_next() {
        Some('+') => {
          self.advance(1);
          return Token::Plus
        },
        Some('-') => {
          self.advance(1);
          return Token::Minus
        },
        Some('*') => {
          self.advance(1);
          return Token::Times
        },
        Some('/') => {
          self.advance(1);
          return Token::Div
        },
        Some('%') => {
          self.advance(1);
          return Token::Mod
        },
        Some('(') => {
          self.advance(1);
          return Token::LParen
        },
        Some(')') => {
          self.advance(1);
          return Token::RParen
        },
        Some('&') => {
          self.advance(2);
          return Token::And
        },
        Some('|') => {
          self.advance(2);
          return Token::Or
        },
        Some('=') if self.text.starts_with("==") => {
          self.advance(2);
          return Token::Eq
        },
        Some('=') => {
          self.advance(1);
          return Token::Assign
        },
        Some('!') if self.text.starts_with("!=") => {
          self.advance(2);
          return Token::Ne
        },
        Some('!') => {
          self.advance(1);
          return Token::Not
        },
        Some('>') if self.text.starts_with(">=") => {
          self.advance(2);
          return Token::Geq
        },
        Some('>') => {
          self.advance(1);
          return Token::Gt
        },
        Some('<') if self.text.starts_with("<=") => {
          self.advance(2);
          return Token::Leq
        },
        Some('<') => {
          self.advance(1);
          return Token::Lt
        },
        Some(';') => {
          self.advance(1);
          return Token::Seq
        },
        Some('?') => {
          self.advance(1);
          return Token::Ternary
        },
        Some(':') => {
          self.advance(1);
          return Token::Else
        },
        Some('{') => {
          self.advance(1);
          return Token::LBracket
        },
        Some('}') => {
          self.advance(1);
          return Token::RBracket
        },
        Some(c) if c.is_alphabetic() => return self.lex_keyword(),
        Some(c) if c.is_digit(10) => return self.lex_integer(),
        Some(c) if c.is_whitespace() => {
          self.skip_whitespace();
          continue;
        },
        None => return Token::EOF,
        _ => panic!()
      }
    }

    Token::EOF
  }
}