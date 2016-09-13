use ast::{Expr, BinOp, UnOp, Dec};
use parser::lexer::Lexer;
use parser::token::Token;
use parser::parser_error::ParserError;
use std::result;

pub type Result<T> = result::Result<T, ParserError>;

struct Parser {
  lexer: Lexer,
  current_token: Token,
}

impl Parser {
  pub fn new(text: String) -> Parser {
    let mut lexer = Lexer::new(text);

    let token = lexer.get_next_token().expect("Lexer error");

    Parser {
      lexer: lexer,
      current_token: token,
    }
  }

  fn current_token(&self) -> Token {
    self.current_token.clone()
  }

  fn eat(&mut self, expected: Token) -> Result<()> {
    let actual = self.current_token();

    if expected != actual {
      return Err(ParserError::UnexpectedToken(expected, actual))
    }

    self.current_token = self.lexer.get_next_token().expect("Lexer error");
    debug!("new current token: {:?}", self.current_token);
    Ok(())
  }

  fn ternary(&mut self, e1: Expr, e2: Expr, e3: Expr) -> Expr {
    Expr::Ternary(to_box(e1), to_box(e2), to_box(e3))
  }

  fn binop(&mut self, bop: BinOp, e1: Expr, e2: Expr) -> Expr {
    Expr::Bop(bop, to_box(e1), to_box(e2))
  }

  fn parse_fn_params(&mut self) -> Result<Vec<Expr>> {
    let mut params = Vec::new();
    let mut token = self.current_token();

    while token != Token::RParen {
      debug!("getting fn params");
      let term = try!(self.binop_expr());

      params.push(term);

      if self.current_token == Token::Comma {
        try!(self.eat(Token::Comma));
      }

      token = self.current_token();
    }

    Ok(params)
  }

  fn parse_fn_decl_params(&mut self) -> Result<Vec<Expr>> {
    let mut params = Vec::new();
    let mut token = self.current_token();

    while token != Token::RParen {
      debug!("getting fn decl params");
      match token.clone() {
        Token::Var(s) => {
          try!(self.eat(Token::Var(s.clone())));
          params.push(Expr::Var(s));
        },
        Token::Comma => try!(self.eat(Token::Comma)),
        _ => return Err(ParserError::InvalidToken(token, String::from("parsing fn decl params")))
      }

      token = self.current_token();
    }

    Ok(params)
  }

  fn parse_fn(&mut self) -> Result<Expr> {
    debug!("parsing named fn...");
    try!(self.eat(Token::FnDecl));

    let var = match self.current_token() {
      Token::Var(s) => {
        try!(self.eat(Token::Var(s.clone())));
        Some(Expr::Var(s))
      },
      _ => None,
    };

    try!(self.eat(Token::LParen));
    let params = try!(self.parse_fn_decl_params());
    try!(self.eat(Token::RParen));

    try!(self.eat(Token::LBracket));
    let body = try!(self.block());
    try!(self.eat(Token::RBracket));

    match var {
      Some(v) => {
        try!(self.eat(Token::Seq));
        let e3 = try!(self.block());

        let func = Expr::Func(Some(to_box(v.clone())), to_box(body.clone()), params);

        Ok(Expr::Decl(Dec::DConst, to_box(v), to_box(func), to_box(e3)))
      },
      None => {
        let func = Expr::Func(None, to_box(body.clone()), params);

        // fn call rule
        if self.current_token == Token::LParen {
          try!(self.eat(Token::LParen));
          let params = try!(self.parse_fn_params());
          try!(self.eat(Token::RParen));

          Ok(Expr::FnCall(to_box(func), params))
        } else {
          Ok(func)
        }
      }
    }
  }

  fn parse_while(&mut self) -> Result<Expr> {
    try!(self.eat(Token::While));
    try!(self.eat(Token::LParen));
    let e1 = try!(self.statement());
    try!(self.eat(Token::RParen));
    let e2 = try!(self.factor());
    try!(self.eat(Token::Seq));
    let e3 = try!(self.block());

    return Ok(Expr::While(
      to_box(e1.clone()),
      to_box(e1),
      to_box(e2.clone()),
      to_box(e2),
      to_box(e3)
    ));
  }

  fn parse_if(&mut self) -> Result<Expr> {
    try!(self.eat(Token::If));
    let e1 = try!(self.binop_expr());
    let e2 = try!(self.block());
    let e3 = try!(self.statement());

    return Ok(self.ternary(e1, e2, e3));
  }

  fn factor(&mut self) -> Result<Expr> {
    let e = match self.current_token() {
      Token::Int(n) => {
        try!(self.eat(Token::Int(n.clone())));
        Expr::Int(n)
      },
      Token::Bool(b) => {
        try!(self.eat(Token::Bool(b.clone())));
        Expr::Bool(b)
      },
      Token::Var(s) => {
        try!(self.eat(Token::Var(s.clone())));

        // fn call rule
        if self.current_token == Token::LParen {
          try!(self.eat(Token::LParen));
          let params = try!(self.parse_fn_params());
          try!(self.eat(Token::RParen));

          Expr::FnCall(to_box(Expr::Var(s)), params)
        } else {
          Expr::Var(s)
        }
      },
      Token::FnDecl => {
        try!(self.parse_fn())
      },
      Token::VarDecl => {
        try!(self.eat(Token::VarDecl));
        let var = try!(self.term());
        try!(self.eat(Token::Assign));
        let e2 = try!(self.statement());
        try!(self.eat(Token::Seq));
        let e3 = try!(self.block());

        Expr::Decl(Dec::DVar, to_box(var), to_box(e2), to_box(e3))
      },
      Token::Let => {
        try!(self.eat(Token::Let));
        let var = try!(self.term());
        try!(self.eat(Token::Assign));
        let e2 = try!(self.statement());
        try!(self.eat(Token::Seq));
        let e3 = try!(self.block());

        Expr::Decl(Dec::DConst, to_box(var), to_box(e2), to_box(e3))
      },
      Token::If => {
        try!(self.parse_if())
      },
      Token::Else => {
        try!(self.eat(Token::Else));
        try!(self.statement())
      },
      Token::While => {
        try!(self.parse_while())
      },
      Token::LParen => {
        try!(self.eat(Token::LParen));
        let node = try!(self.statement());
        try!(self.eat(Token::RParen));
        node
      },
      Token::LBracket => {
        try!(self.eat(Token::LBracket));
        let node = try!(self.block());
        try!(self.eat(Token::RBracket));
        node
      },
      Token::Not => {
        try!(self.eat(Token::Not));
        Expr::Uop(UnOp::Not, to_box(try!(self.factor())))
      },
      Token::Minus => {
        try!(self.eat(Token::Minus));
        Expr::Uop(UnOp::Neg, to_box(try!(self.factor())))
      },
      Token::EOF => {
        try!(self.eat(Token::EOF));
        Expr::Undefined
      },
      _ => {
        return Err(ParserError::InvalidToken(self.current_token(), String::from("parsing factor")))
      }
    };

    Ok(e)
  }

  pub fn term(&mut self) -> Result<Expr> {
    let mut node = try!(self.factor());

    let mut op = self.current_token();

    while op.is_term_op() {
      try!(self.eat(op.clone()));
      let right_node = try!(self.term());

      node = match op {
        Token::Times => self.binop(BinOp::Times, node, right_node),
        Token::Div => self.binop(BinOp::Div, node, right_node),
        _ => panic!(),
      };

      op = self.current_token();
    }

    Ok(node)
  }

  pub fn binop_expr(&mut self) -> Result<Expr> {
    let mut node = try!(self.term());

    let mut op = self.current_token();

    while op.is_expr_op() {
      debug!("expr looping on op {:?}", op);
      try!(self.eat(op.clone()));

      let right_node = try!(self.term());

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

      op = self.current_token();
    }
    
    Ok(node)
  }

  pub fn statement(&mut self) -> Result<Expr> {
    let mut node = try!(self.binop_expr());
    let mut op = self.current_token();

    while op.is_statement_op() {
      try!(self.eat(op.clone()));

      node = match op {
        Token::Ternary => {
          let e2 = try!(self.block());
          try!(self.eat(Token::Else));
          let e3 = try!(self.statement());
          self.ternary(node, e2, e3)
        },
        Token::Assign => {
          let e2 = try!(self.statement());
          self.binop(BinOp::Assign, node, e2)
        }
        _ => panic!()
      };

      op = self.current_token();
    }

    Ok(node)
  }

  pub fn block(&mut self) -> Result<Expr> {
    let mut node = try!(self.statement());
    let mut op = self.current_token();

    while op.is_block_op() {
      try!(self.eat(op.clone()));

      node = match op {
        Token::Seq => {
          let e2 = match self.current_token() {
            Token::RBracket => Expr::Undefined,
            _ => try!(self.statement()),
          };

          self.binop(BinOp::Seq, node, e2)
        }
        _ => panic!()
      };

      op = self.current_token();
    }

    Ok(node)
  }

  pub fn program(&mut self) -> Result<Expr> {
    self.block()
  }
}

fn to_box(e: Expr) -> Box<Expr> {
  Box::new(e)
}

pub fn parse(input: &str) -> Result<Expr> {
  let mut parser = Parser::new(input.to_string());
  let expr = parser.program();

  debug!("parsed expr: {:#?}", expr);
  debug!("original: {:#?}", input);

  expr
}
