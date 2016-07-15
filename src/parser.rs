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

  pub fn is_expr_op(&self) -> bool {
    match *self {
      Token::Ternary => true,
      Token::Seq => true,
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

struct Parser {
  lexer: Lexer,
  current_token: Token,
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

  fn eat(&mut self, expected: Token) {
    let actual = self.current_token.clone();

    if expected != actual {
      panic!("expected token: {:?} actual: {:?}", expected, actual)
    }

    self.current_token = self.lexer.get_next_token();
    debug!("new current token: {:?}", self.current_token);
  }

  fn ternary(&mut self, e1: Expr, e2: Expr, e3: Expr) -> Expr {
    Expr::Ternary(to_box(e1), to_box(e2), to_box(e3))
  }

  fn binop(&mut self, bop: BinOp, e1: Expr, e2: Expr) -> Expr {
    Expr::BinOp(bop, to_box(e1), to_box(e2))
  }

  fn factor(&mut self) -> Expr {
    match self.current_token.clone() {
      Token::Int(n) => {
        self.eat(Token::Int(n.clone()));
        return Expr::Int(n);
      },
      Token::Bool(b) => {
        self.eat(Token::Bool(b.clone()));
        return Expr::Bool(b);
      },
      Token::Var(s) => {
        self.eat(Token::Var(s.clone()));
        return Expr::Var(s);
      },
      Token::FnDecl => {
        self.eat(Token::FnDecl);

        self.eat(Token::LParen);

        // TODO: grab params here

        self.eat(Token::RParen);

        self.eat(Token::LBracket);
        let body = self.statement();
        debug!("got fn body {:?}", body);
        self.eat(Token::RBracket);

        return Expr::Func(to_box(body));
      },
      Token::FnCall(s) => {
        self.eat(Token::FnCall(s.clone()));
        // TODO: grab any params

        self.eat(Token::LParen);
        self.eat(Token::RParen);

        return Expr::FnCall(s);
      },
      Token::LParen => {
        self.eat(Token::LParen);
        let node = self.statement();
        self.eat(Token::RParen);
        return node;
      },
      Token::LBracket => {
        self.eat(Token::LBracket);
        let node = self.statement();
        self.eat(Token::RBracket);
        return node;
      },
      Token::Not => {
        self.eat(Token::Not);
        return Expr::UnOp(UnOp::Not, to_box(self.statement()));
      },
      Token::Minus => {
        self.eat(Token::Minus);
        return Expr::UnOp(UnOp::Neg, to_box(self.factor()));
      },
      _ => {
        debug!("invalid factor: {:?}", self.current_token);
        panic!();
      },
    }
  }

  pub fn term(&mut self) -> Expr {
    let mut node = self.factor();

    let mut op = self.current_token.clone();

    while op.clone().is_term_bop() {
      self.eat(op.clone());
      let right_node = self.term();

      node = match op {
        Token::Times => self.binop(BinOp::Times, node, right_node),
        Token::Div => self.binop(BinOp::Div, node, right_node),
        _ => panic!(),
      };

      op = self.current_token.clone();
    }

    node
  }

  pub fn binop_expr(&mut self) -> Expr {
    let mut node = self.term();

    let mut op = self.current_token.clone();

    while op.clone().is_expr_bop() {
      debug!("expr looping on op {:?}", op);
      self.eat(op.clone());

      let right_node = self.term();

      node = match op {
        Token::Plus => self.binop(BinOp::Plus, node, right_node),
        Token::Minus => self.binop(BinOp::Minus, node, right_node),
        Token::Eq => self.binop(BinOp::Eq, node, right_node),
        Token::Ne => self.binop(BinOp::Ne, node, right_node),
        Token::Leq => self.binop(BinOp::Leq, node, right_node),
        Token::Geq => self.binop(BinOp::Geq, node, right_node),
        Token::Lt => self.binop(BinOp::Lt, node, right_node),
        Token::Gt => self.binop(BinOp::Gt, node, right_node),
        Token::And => self.binop(BinOp::And, node, right_node),
        Token::Or => self.binop(BinOp::Or, node, right_node),
        Token::Mod => self.binop(BinOp::Mod, node, right_node),
        _ => panic!(),
      };

      op = self.current_token.clone();
    }
    
    node 
  }

  fn parse_let(&mut self) -> Expr {
    self.eat(Token::Let);
    let var = self.term();
    self.eat(Token::Assign);
    let e2 = self.binop_expr();
    self.eat(Token::Seq);
    let e3 = self.statement();

    return Expr::Let(Box::new(var), Box::new(e2), Box::new(e3));
  }

  pub fn statement(&mut self) -> Expr {
    if self.current_token == Token::Let {
      return self.parse_let();
    }

    let mut node = self.binop_expr();

    let mut op = self.current_token.clone();

    while op.clone().is_expr_op() {
      self.eat(op.clone());
      let e2 = self.statement();

      node = match op {
        Token::Ternary => {

          self.eat(Token::Else);

          let e3 = self.statement();

          self.ternary(node, e2, e3)
        },
        Token::Seq => {
          self.binop(BinOp::Seq, node, e2)
        }
        _ => panic!(),
      };

      op = self.current_token.clone();
    }

    node
  }

  pub fn block(&mut self) -> Expr {
    self.statement()
  }

  pub fn program(&mut self) -> Expr {
    self.block()
  }
}

fn to_box(e: Expr) -> Box<Expr> {
  Box::new(e)
}


pub fn parse(input: &str) -> Expr {
  let mut parser = Parser::new(input.to_string());
  let expr = parser.program();

  debug!("parsed expr: {:#?}", expr);

  expr
}
