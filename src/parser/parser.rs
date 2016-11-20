use expr::{Expr, BinOp, UnOp, Dec};
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
  pub fn new(lexer: Lexer, token: Token) -> Parser {
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

    self.current_token = self.lexer.get_next_token()?;
    debug!("new current token: {:?}", self.current_token);
    Ok(())
  }

  fn ternary(&mut self, e1: Expr, e2: Expr, e3: Expr) -> Expr {
    Expr::Ternary(Box::new(e1), Box::new(e2), Box::new(e3))
  }

  fn binop(&mut self, bop: BinOp, e1: Expr, e2: Expr) -> Expr {
    Expr::Bop(bop, Box::new(e1), Box::new(e2))
  }

  fn parse_fn_params(&mut self) -> Result<Vec<Expr>> {
    let mut params = Vec::new();
    let mut token = self.current_token();

    while token != Token::RParen {
      debug!("getting fn params");
      let term = self.binop_expr()?;

      params.push(term);

      if self.current_token == Token::Comma {
        self.eat(Token::Comma)?;
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
          self.eat(Token::Var(s.clone()))?;
          params.push(Expr::Var(s));
        },
        Token::Comma => self.eat(Token::Comma)?,
        _ => return Err(ParserError::InvalidToken(token, String::from("parsing fn decl params")))
      }

      token = self.current_token();
    }

    Ok(params)
  }

  fn parse_print(&mut self) -> Result<Expr> {
    self.eat(Token::Print)?;
    let term = self.binop_expr()?;
    Ok(Expr::Print(Box::new(term)))
  }

  fn parse_print_var_name(&mut self) -> Result<Expr> {
    self.eat(Token::PrintVarName)?;
    self.eat(Token::LParen)?;

    match self.current_token() {
      Token::Var(s) => {
        self.eat(Token::Var(s.clone()))?;
        self.eat(Token::RParen)?;
        Ok(Expr::PrintVarName(Box::new(Expr::Var(s))))
      },
      t => Err(ParserError::InvalidToken(t, String::from("parsing name for PrintVarName")))
    }
  }

  fn parse_fn(&mut self) -> Result<Expr> {
    debug!("parsing named fn...");
    self.eat(Token::FnDecl)?;

    let var = match self.current_token() {
      Token::Var(s) => {
        self.eat(Token::Var(s.clone()))?;
        Some(Expr::Var(s))
      },
      _ => None,
    };

    self.eat(Token::LParen)?;
    let params = self.parse_fn_decl_params()?;
    self.eat(Token::RParen)?;

    self.eat(Token::LBracket)?;
    let body = self.block()?;
    self.eat(Token::RBracket)?;

    match var {
      Some(v) => {
        self.eat(Token::Seq)?;
        let e3 = self.block()?;

        let func = Expr::Func(Some(Box::new(v.clone())), Box::new(body.clone()), params);

        Ok(Expr::Decl(Dec::DConst, Box::new(v), Box::new(func), Box::new(e3)))
      },
      None => {
        let func = Expr::Func(None, Box::new(body.clone()), params);

        // fn call rule
        if self.current_token == Token::LParen {
          self.eat(Token::LParen)?;
          let params = self.parse_fn_params()?;
          self.eat(Token::RParen)?;

          Ok(Expr::FnCall(Box::new(func), params))
        } else {
          Ok(func)
        }
      }
    }
  }

  fn parse_while(&mut self) -> Result<Expr> {
    self.eat(Token::While)?;
    self.eat(Token::LParen)?;
    let e1 = self.statement()?;
    self.eat(Token::RParen)?;
    let e2 = self.factor()?;
    self.eat(Token::Seq)?;
    let e3 = self.block()?;

    return Ok(Expr::While(
      Box::new(e1.clone()),
      Box::new(e1),
      Box::new(e2.clone()),
      Box::new(e2),
      Box::new(e3)
    ));
  }

  fn parse_if(&mut self) -> Result<Expr> {
    self.eat(Token::If)?;
    let e1 = self.binop_expr()?;
    let e2 = self.block()?;
    let e3 = self.statement()?;

    return Ok(self.ternary(e1, e2, e3));
  }

  fn factor(&mut self) -> Result<Expr> {
    let e = match self.current_token() {
      Token::Int(n) => {
        self.eat(Token::Int(n.clone()))?;
        Expr::Int(n)
      },
      Token::Bool(b) => {
        self.eat(Token::Bool(b.clone()))?;
        Expr::Bool(b)
      },
      Token::Var(s) => {
        self.eat(Token::Var(s.clone()))?;

        // fn call rule
        if self.current_token == Token::LParen {
          self.eat(Token::LParen)?;
          let params = self.parse_fn_params()?;
          self.eat(Token::RParen)?;

          Expr::FnCall(Box::new(Expr::Var(s)), params)
        } else {
          Expr::Var(s)
        }
      },
      Token::Give => {
        self.eat(Token::Give)?;
        self.eat(Token::LParen)?;
        match self.current_token() {
          Token::Var(s) => {
            self.eat(Token::Var(s.clone()))?;
            self.eat(Token::RParen)?;
            Ok(Expr::Give(Box::new(Expr::Var(s))))
          },
          t => Err(ParserError::InvalidToken(t, String::from("parsing name for give")))
        }?
      },
      Token::Print => {
        self.parse_print()?
      },
      Token::PrintVarName => {
        self.parse_print_var_name()?
      },
      Token::FnDecl => {
        self.parse_fn()?
      },
      Token::VarDecl => {
        self.eat(Token::VarDecl)?;
        let var = self.term()?;
        self.eat(Token::Assign)?;
        let e2 = self.statement()?;
        self.eat(Token::Seq)?;
        let e3 = self.block()?;

        Expr::Decl(Dec::DVar, Box::new(var), Box::new(e2), Box::new(e3))
      },
      Token::Let => {
        self.eat(Token::Let)?;
        let var = self.term()?;
        self.eat(Token::Assign)?;
        let e2 = self.statement()?;
        self.eat(Token::Seq)?;
        let e3 = self.block()?;

        Expr::Decl(Dec::DConst, Box::new(var), Box::new(e2), Box::new(e3))
      },
      Token::If => {
        self.parse_if()?
      },
      Token::Rebattle => {
        self.eat(Token::Rebattle)?;

        let e1 = self.binop_expr()?;
        let e2 = self.block()?;
        let e3 = self.statement()?;

        self.ternary(e1, e2, e3)
      },
      Token::Else => {
        self.eat(Token::Else)?;
        self.statement()?
      },
      Token::While => {
        self.parse_while()?
      },
      Token::LParen => {
        self.eat(Token::LParen)?;
        let node = self.statement()?;
        self.eat(Token::RParen)?;
        node
      },
      Token::LBracket => {
        self.eat(Token::LBracket)?;
        let node = self.block()?;
        self.eat(Token::RBracket)?;
        node
      },
      Token::Not => {
        self.eat(Token::Not)?;
        Expr::Uop(UnOp::Not, Box::new(self.factor()?))
      },
      Token::Minus => {
        self.eat(Token::Minus)?;
        Expr::Uop(UnOp::Neg, Box::new(self.factor()?))
      },
      Token::EOF => {
        self.eat(Token::EOF)?;
        Expr::Undefined
      },
      _ => {
        return Err(ParserError::InvalidToken(self.current_token(), String::from("parsing factor")))
      }
    };

    Ok(e)
  }

  pub fn term(&mut self) -> Result<Expr> {
    let mut node = self.factor()?;

    let mut op = self.current_token();

    while op.is_term_op() {
      self.eat(op.clone())?;
      let right_node = self.term()?;

      node = match op {
        Token::Times => self.binop(BinOp::Times, node, right_node),
        Token::Div => self.binop(BinOp::Div, node, right_node),
        _ => return Err(ParserError::InvalidToken(op, String::from("parsing term")))
      };

      op = self.current_token();
    }

    Ok(node)
  }

  pub fn binop_expr(&mut self) -> Result<Expr> {
    let mut node = self.term()?;

    let mut op = self.current_token();

    while op.is_expr_op() {
      debug!("expr looping on op {:?}", op);
      self.eat(op.clone())?;

      let right_node = self.term()?;

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
        _ => return Err(ParserError::InvalidToken(op, String::from("parsing binop expression")))
      };

      op = self.current_token();
    }
    
    Ok(node)
  }

  pub fn statement(&mut self) -> Result<Expr> {
    let mut node = self.binop_expr()?;
    let mut op = self.current_token();

    while op.is_statement_op() {
      self.eat(op.clone())?;

      node = match op {
        Token::Ternary => {
          let e2 = self.block()?;
          self.eat(Token::Else)?;
          let e3 = self.statement()?;
          self.ternary(node, e2, e3)
        },
        Token::Assign => {
          let e2 = self.statement()?;
          self.binop(BinOp::Assign, node, e2)
        },
        _ => return Err(ParserError::InvalidToken(op, String::from("parsing statement")))
      };

      op = self.current_token();
    }

    Ok(node)
  }

  pub fn block(&mut self) -> Result<Expr> {
    let mut node = self.statement()?;
    let mut op = self.current_token();

    while op.is_block_op() {
      self.eat(op.clone())?;

      node = match op {
        Token::Seq => {
          let e2 = match self.current_token() {
            Token::RBracket => Expr::Undefined,
            _ => self.statement()?,
          };

          self.binop(BinOp::Seq, node, e2)
        },
        _ => return Err(ParserError::InvalidToken(op, String::from("parsing block")))
      };

      op = self.current_token();
    }

    Ok(node)
  }

  pub fn program(&mut self) -> Result<Expr> {
    self.block()
  }
}

pub fn parse(input: &str) -> Result<Expr> {
  let mut lexer = Lexer::new(input.to_string());

  let token = lexer.get_next_token()?;
  
  let mut parser = Parser::new(lexer, token);
  let expr = parser.program();

  debug!("parsed expr: {:#?}", expr);
  debug!("original: {:#?}", input);

  expr
}
