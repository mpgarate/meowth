use expr::{Expr, BinOp, UnOp};
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

    while op.is_statement_op() {
      self.eat(op.clone());
      let e2 = self.statement();

      node = match op {
        Token::Ternary => {
          self.eat(Token::Else);
          let e3 = self.statement();
          self.ternary(node, e2, e3)
        },
        Token::Seq => self.binop(BinOp::Seq, node, e2),
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
