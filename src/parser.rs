use ast::{Expr, BinOp, UnOp, Dec};
use lexer::{Lexer, Token};

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
    Expr::Bop(bop, to_box(e1), to_box(e2))
  }

  fn parse_fn_params(&mut self) -> Vec<Expr> {
    let mut params = Vec::new();
    let mut token = self.current_token.clone();

    while token != Token::RParen {
      debug!("getting fn params");
      let term = self.binop_expr();

      params.push(term);

      if self.current_token == Token::Comma {
        self.eat(Token::Comma);
      }

      token = self.current_token.clone();
    }

    params
  }

  fn parse_fn_decl_params(&mut self) -> Vec<Expr> {
    let mut params = Vec::new();
    let mut token = self.current_token.clone();

    while token != Token::RParen {
      debug!("getting fn decl params");
      match token.clone() {
        Token::Var(s) => {
          self.eat(Token::Var(s.clone()));
          params.push(Expr::Var(s));
        },
        Token::Comma => self.eat(Token::Comma),
        _ => panic!()
      }

      token = self.current_token.clone();
    }

    params
  }

  fn parse_fn(&mut self) -> Expr {
    debug!("parsing named fn...");
    self.eat(Token::FnDecl);

    let var = match self.current_token.clone() {
      Token::Var(s) => {
        self.eat(Token::Var(s.clone()));
        Some(Expr::Var(s))
      },
      _ => None,
    };

    self.eat(Token::LParen);
    let params = self.parse_fn_decl_params();
    self.eat(Token::RParen);
    self.eat(Token::LBracket);
    let body = self.statement();
    self.eat(Token::RBracket);

    match var {
      Some(v) => {
        self.eat(Token::Seq);
        let e3 = self.statement();

        let func = Expr::Func(Some(to_box(v.clone())), to_box(body.clone()), params);

        Expr::Decl(Dec::DConst, to_box(v), to_box(func), to_box(e3))
      },
      None => {
        Expr::Func(None, to_box(body.clone()), params)
      }
    }
  }

  fn parse_if(&mut self) -> Expr {
    self.eat(Token::If);
    let e1 = self.binop_expr();
    let e2 = self.statement();
    let e3 = self.statement();
    return self.ternary(e1, e2, e3);
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

        // fn call rule
        if self.current_token == Token::LParen {
          self.eat(Token::LParen);
          let params = self.parse_fn_params();
          self.eat(Token::RParen);

          return Expr::FnCall(to_box(Expr::Var(s)), params);
        }

        return Expr::Var(s);
      },
      Token::FnDecl => {
        return self.parse_fn();
      },
      Token::VarDecl => {
        self.eat(Token::VarDecl);
        let var = self.term();
        self.eat(Token::Assign);
        let e2 = self.statement();
        self.eat(Token::Seq);
        let e3 = self.block();

        return Expr::Decl(Dec::DVar, to_box(var), to_box(e2), to_box(e3));
      },
      Token::Let => {
        self.eat(Token::Let);
        let var = self.term();
        self.eat(Token::Assign);
        let e2 = self.statement();
        self.eat(Token::Seq);
        let e3 = self.block();

        return Expr::Decl(Dec::DConst, to_box(var), to_box(e2), to_box(e3));
      },
      Token::If => {
        return self.parse_if();
      },
      Token::Else => {
        self.eat(Token::Else);
        return self.statement();
      },
      Token::LParen => {
        self.eat(Token::LParen);
        let node = self.statement();
        self.eat(Token::RParen);
        return node;
      },
      Token::LBracket => {
        self.eat(Token::LBracket);
        let node = self.block();
        self.eat(Token::RBracket);
        return node;
      },
      Token::Not => {
        self.eat(Token::Not);
        return Expr::Uop(UnOp::Not, to_box(self.factor()));
      },
      Token::Minus => {
        self.eat(Token::Minus);
        return Expr::Uop(UnOp::Neg, to_box(self.factor()));
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

    while op.is_term_op() {
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

    while op.is_expr_op() {
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
        Token::Assign => self.binop(BinOp::Assign, node, right_node),
        _ => panic!(),
      };

      op = self.current_token.clone();
    }
    
    node 
  }

  pub fn statement(&mut self) -> Expr {
    let mut node = self.binop_expr();
    let mut op = self.current_token.clone();

    while op.is_statement_op() {
      self.eat(op.clone());

      node = match op {
        Token::Ternary => {
          let e2 = self.block();
          self.eat(Token::Else);
          let e3 = self.statement();
          self.ternary(node, e2, e3)
        },
        _ => panic!()
      };

      op = self.current_token.clone();
    }

    node
  }

  pub fn block(&mut self) -> Expr {
    let mut node = self.statement();
    let mut op = self.current_token.clone();

    while op.is_block_op() {
      self.eat(op.clone());
      let e2 = self.statement();

      node = match op {
        Token::Seq => self.binop(BinOp::Seq, node, e2),
        _ => panic!()
      };

      op = self.current_token.clone();
    }

    node
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
  debug!("original: {:#?}", input);

  expr
}
