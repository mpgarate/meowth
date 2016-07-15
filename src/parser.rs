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
      Token::Seq => true,
      _ => false,
    }
  }

  pub fn is_expr_op(&self) -> bool {
    match *self {
      Token::Ternary => true,
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

    let next_char = self.peek_next();

    match keyword.as_ref()  {
      "true" => Some(Token::Bool(true)),
      "false" => Some(Token::Bool(false)),
      "fn" => Some(Token::FnDecl),
      "let" => Some(Token::Let),
      s if next_char == Some('(') => Some(Token::FnCall(s.to_string())),
      s if s.len() > 0 => Some(Token::Var(s.to_string())),
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
        Some('=') => {
          self.advance(1);
          return Some(Token::Assign)
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
        Some(';') => {
          self.advance(1);
          return Some(Token::Seq)
        },
        Some('?') => {
          self.advance(1);
          return Some(Token::Ternary)
        },
        Some(':') => {
          self.advance(1);
          return Some(Token::Else)
        },
        Some('{') => {
          self.advance(1);
          return Some(Token::LBracket)
        },
        Some('}') => {
          self.advance(1);
          return Some(Token::RBracket)
        },
        Some(c) if c.is_alphabetic() => return self.lex_keyword(),
        Some(c) if c.is_digit(10) => return self.lex_integer(),
        Some(c) if c.is_whitespace() => {
          self.skip_whitespace();
          continue;
        },
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

  fn eat(&mut self, expected: Token) {
    let actual = self.current_token.clone().unwrap();

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
      Some(Token::Int(n)) => {
        self.eat(Token::Int(n.clone()));
        return Expr::Int(n);
      },
      Some(Token::Bool(b)) => {
        self.eat(Token::Bool(b.clone()));
        return Expr::Bool(b);
      },
      Some(Token::Var(s)) => {
        self.eat(Token::Var(s.clone()));
        return Expr::Var(s);
      },
      Some(Token::Let) => {
        self.eat(Token::Let);

        let var = self.statement();

        self.eat(Token::Assign);

        let seq = self.statement();

        // TODO: this is def hacky
        let (e1, e2) = match seq {
          Expr::BinOp(BinOp::Seq, e1, e2) => {
            (e1, e2)
          },
          _ => {
            debug!("expected seq, got {:?}", seq);
            panic!();
          }
        };

        debug!("var: {:?}", var);

        return Expr::Let(to_box(var), e1, e2);
      },
      Some(Token::FnDecl) => {
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
      Some(Token::FnCall(s)) => {
        self.eat(Token::FnCall(s.clone()));
        // TODO: grab any params

        self.eat(Token::LParen);
        self.eat(Token::RParen);

        return Expr::FnCall(s);
      },
      Some(Token::LParen) => {
        self.eat(Token::LParen);
        let node = self.statement();
        self.eat(Token::RParen);
        return node;
      },
      Some(Token::LBracket) => {
        self.eat(Token::LBracket);
        let node = self.statement();
        self.eat(Token::RBracket);
        return node;
      },
      Some(Token::Not) => {
        self.eat(Token::Not);
        return Expr::UnOp(UnOp::Not, to_box(self.statement()));
      },
      Some(Token::Minus) => {
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

    while op != None && op.clone().unwrap().is_term_bop() {
      self.eat(op.clone().unwrap());
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

  pub fn binop_expr(&mut self) -> Expr {
    let mut node = self.term();

    let mut op = self.current_token.clone();

    while op != None && op.clone().unwrap().is_expr_bop() {
      debug!("expr looping on op {:?}", op);
      self.eat(op.clone().unwrap());

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
        Some(Token::Seq) => self.binop(BinOp::Seq, node, right_node),
        _ => panic!(),
      };

      op = self.current_token.clone();
    }
    
    node 
  }

  pub fn statement(&mut self) -> Expr {
    let mut node = self.binop_expr();

    let mut op = self.current_token.clone();

    while op != None && op.clone().unwrap().is_expr_op() {
      self.eat(op.clone().unwrap());
      let e2 = self.binop_expr();

      node = match op {
        Some(Token::Ternary) => {

          self.eat(Token::Else);

          let e3 = self.binop_expr();

          self.ternary(node, e2, e3)
        },
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
