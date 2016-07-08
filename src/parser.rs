use expr::{Expr, BinOp, UnOp};

#[derive(Clone, Debug, PartialEq)] 
enum Token {
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
  Int(isize),
  Bool(bool),
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
}

struct Lexer {
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

  fn lex_integer(&mut self) -> Option<Token> {
    let int_str: String = self.text
      .chars()
      .take_while(|c| c.is_digit(10))
      .collect();

    match int_str.parse::<isize>() {
      Ok(n) => {
        self.advance(int_str.len());
        return Some(Token::Int(n));
      }
      Err(_) => panic!()
    }
  }

  fn lex_keyword(&mut self) -> Option<Token> {
    let keyword: String = self.text
      .chars()
      .take_while(|c| c.is_alphabetic())
      .collect();

    self.advance(keyword.len());

    match keyword.as_ref()  {
      "true" => Some(Token::Bool(true)),
      "false" => Some(Token::Bool(false)),
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

  pub fn get_next_token(&mut self) -> Option<Token> {
    while self.peek_next() != None {
      debug!("get_next_token: {}", self.text);

      match self.peek_next() {
        Some('+') => {
          self.advance(1);
          return Some(Token::Plus)
        },
        Some('-') => {
          self.advance(1);
          return Some(Token::Minus)
        },
        Some('*') => {
          self.advance(1);
          return Some(Token::Times)
        },
        Some('/') => {
          self.advance(1);
          return Some(Token::Div)
        },
        Some('%') => {
          self.advance(1);
          return Some(Token::Mod)
        },
        Some('(') => {
          self.advance(1);
          return Some(Token::LParen)
        },
        Some(')') => {
          self.advance(1);
          return Some(Token::RParen)
        },
        Some('&') => {
          self.advance(2);
          return Some(Token::And)
        },
        Some('|') => {
          self.advance(2);
          return Some(Token::Or)
        },
        Some('=') if self.text.starts_with("==") => {
          self.advance(2);
          return Some(Token::Eq)
        },
        Some('!') if self.text.starts_with("!=") => {
          self.advance(2);
          return Some(Token::Ne)
        },
        Some('!') => {
          self.advance(1);
          return Some(Token::Not)
        },
        Some('>') if self.text.starts_with(">=") => {
          self.advance(2);
          return Some(Token::Geq)
        },
        Some('>') => {
          self.advance(1);
          return Some(Token::Gt)
        },
        Some('<') if self.text.starts_with("<=") => {
          self.advance(2);
          return Some(Token::Leq)
        },
        Some('<') => {
          self.advance(1);
          return Some(Token::Lt)
        },
        Some(c) if c.is_alphabetic() => return self.lex_keyword(),
        Some(c) if c.is_digit(10) => return self.lex_integer(),
        Some(c) if c.is_whitespace() => {
          self.skip_whitespace();
          continue;
        }
        None => return None,
        _ => panic!()
      }
    }

    None
  }
}

struct Parser {
  lexer: Lexer,
  current_token: Option<Token>,
}

impl Parser {
  pub fn new(text: String) -> Parser {
    let mut lexer = Lexer::new(text);

    let token = lexer.get_next_token();

    Parser {
      lexer: lexer,
      current_token: token,
    }
  }

  fn eat(&mut self) {
    self.current_token = self.lexer.get_next_token();
    debug!("new current token: {:?}", self.current_token);
  }

  fn binop(&mut self, bop: BinOp, e1: Option<Expr>, e2: Option<Expr>) -> Option<Expr> {
    Some(Expr::BinOp(bop, Box::new(e1.unwrap()), Box::new(e2.unwrap())))
  }

  fn factor(&mut self) -> Option<Expr> {
    match self.current_token {
      Some(Token::Int(n)) => {
        self.eat();
        return Some(Expr::Int(n));
      },
      Some(Token::Bool(b)) => {
        self.eat();
        return Some(Expr::Bool(b));
      },
      Some(Token::LParen) => {
        self.eat();
        let node = self.expr();
        self.eat();
        return node;
      },
      Some(Token::Not) => {
        self.eat();
        return Some(Expr::UnOp(UnOp::Not, Box::new(self.expr().unwrap())))
      },
      Some(Token::Minus) => {
        self.eat();
        return Some(Expr::UnOp(UnOp::Neg, Box::new(self.factor().unwrap())))
      },
      _ => {
        debug!("invalid factor: {:?}", self.current_token);
        panic!();
      },
    }
  }

  pub fn term(&mut self) -> Option<Expr> {
    let mut node = self.factor();

    let mut op = self.current_token.clone();

    while op != None && op.clone().unwrap().is_term_bop() {
      self.eat();
      let right_node = self.term();

      node = match op {
        Some(Token::Times) => self.binop(BinOp::Times, node, right_node),
        Some(Token::Div) => self.binop(BinOp::Div, node, right_node),
        _ => panic!(),
      };

      op = self.current_token.clone();
    }

    node
  }

  pub fn expr(&mut self) -> Option<Expr> {
    let mut node = self.term();

    let mut op = self.current_token.clone();

    while op != None && op.clone().unwrap().is_expr_bop() {
      self.eat();
      let right_node = self.term();

      node = match op {
        Some(Token::Plus) => self.binop(BinOp::Plus, node, right_node),
        Some(Token::Minus) => self.binop(BinOp::Minus, node, right_node),
        Some(Token::Eq) => self.binop(BinOp::Eq, node, right_node),
        Some(Token::Ne) => self.binop(BinOp::Ne, node, right_node),
        Some(Token::Leq) => self.binop(BinOp::Leq, node, right_node),
        Some(Token::Geq) => self.binop(BinOp::Geq, node, right_node),
        Some(Token::Lt) => self.binop(BinOp::Lt, node, right_node),
        Some(Token::Gt) => self.binop(BinOp::Gt, node, right_node),
        Some(Token::And) => self.binop(BinOp::And, node, right_node),
        Some(Token::Or) => self.binop(BinOp::Or, node, right_node),
        Some(Token::Mod) => self.binop(BinOp::Mod, node, right_node),
        _ => panic!(),
      };

      op = self.current_token.clone();
    }
    
    node 
  }
}

pub fn parse(input: &str) -> Expr {
  let mut parser = Parser::new(input.to_string());
  let expr = parser.expr().unwrap();

  debug!("parsed expr: {:?}", expr);

  expr
}
