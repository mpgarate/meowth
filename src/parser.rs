use expr::{Expr, BinOp};

#[derive(Clone, Debug, PartialEq)] 
enum Token {
  Plus,
  Minus,
  Times,
  Div,
  RParen,
  LParen,
  Integer(isize),
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

  fn cut_input_by(&mut self, n: usize) {
    let text = self.text.clone();
    let (_, t) = text.split_at(n);
    self.text = t.to_string();
  }

  pub fn lex_integer(&mut self) -> Option<Token> {
    let int_str: String = self.text
      .chars()
      .take_while(|c| c.is_digit(10))
      .collect();

    match int_str.parse::<isize>() {
      Ok(n) => {
        self.cut_input_by(int_str.len());
        return Some(Token::Integer(n));
      }
      Err(_) => panic!()
    }
  }

  fn skip_whitespace(&mut self) {
    let mut c = self.text.chars().next();

    loop {
      debug!("skipping whitespace... {:?}", c);
      self.cut_input_by(1);
      c = self.text.chars().next();

      if c == None || !c.unwrap().is_whitespace() {
        break;
      }
    }
  }

  pub fn get_next_token(&mut self) -> Option<Token> {
    while self.text.chars().next() != None {
      debug!("get_next_token: {}", self.text);
      match self.text.chars().next() {
        Some('+') => {
          self.cut_input_by(1);
          return Some(Token::Plus)
        },
        Some('-') => {
          self.cut_input_by(1);
          return Some(Token::Minus)
        },
        Some('*') => {
          self.cut_input_by(1);
          return Some(Token::Times)
        },
        Some('/') => {
          self.cut_input_by(1);
          return Some(Token::Div)
        },
        Some('(') => {
          self.cut_input_by(1);
          return Some(Token::LParen)
        },
        Some(')') => {
          self.cut_input_by(1);
          return Some(Token::RParen)
        },
        Some(x) if x.is_digit(10) => return self.lex_integer(),
        Some(x) if x.is_whitespace() => {
          self.skip_whitespace();
          continue;
        }
        None => {
          debug!("Lex'd none! EOF!");
          return None
        }
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

  fn plus(&mut self, e1: Option<Expr>, e2: Option<Expr>) -> Option<Expr> {
    match (e1, e2) {
      (Some(e1), Some(e2)) => Some(Expr::BinOp(BinOp::Plus,Box::new(e1), Box::new(e2))),
      _ => panic!()
    }
  }

  fn minus (&mut self, e1: Option<Expr>, e2: Option<Expr>) -> Option<Expr> {
    match (e1, e2) {
      (Some(e1), Some(e2)) => Some(Expr::BinOp(BinOp::Minus,Box::new(e1), Box::new(e2))),
      _ => panic!()
    }
  }

  fn times (&mut self, e1: Option<Expr>, e2: Option<Expr>) -> Option<Expr> {
    match (e1, e2) {
      (Some(e1), Some(e2)) => Some(Expr::BinOp(BinOp::Times,Box::new(e1), Box::new(e2))),
      _ => panic!()
    }
  }

  fn div (&mut self, e1: Option<Expr>, e2: Option<Expr>) -> Option<Expr> {
    match (e1, e2) {
      (Some(e1), Some(e2)) => Some(Expr::BinOp(BinOp::Div,Box::new(e1), Box::new(e2))),
      _ => panic!()
    }
  }

  fn factor(&mut self) -> Option<Expr> {
    match self.current_token {
      Some(Token::Integer(n)) => {
        debug!("factor::Integer({})", n);
        self.eat();
        return Some(Expr::Int(n));
      },
      Some(Token::LParen) => {
        debug!("factor::LParen)");
        self.eat();
        let node = self.expr();
        self.eat();
        return node;
      },
      _ => {
        debug!("invalid factor: {:?}", self.current_token);
        panic!();
      },
    }
  }

  pub fn term(&mut self) -> Option<Expr> {
    let mut node = self.factor();

    while self.current_token == Some(Token::Times) || self.current_token == Some(Token::Div) {
      let op = self.current_token.clone();

      self.eat();
      let right_node = self.term();

      node = match op {
        Some(Token::Times) => self.times(node, right_node),
        Some(Token::Div) => self.div(node, right_node),
        _ => panic!(),
      };
    }

    node
  }

  pub fn expr(&mut self) -> Option<Expr> {
    debug!("");
    debug!("left_node: (");
    let mut node = self.term();
    debug!(") // left_node");
    debug!("");

    while self.current_token == Some(Token::Plus) || self.current_token == Some(Token::Minus) {
      debug!("expr::Op({:?})", self.current_token);

      let op = self.current_token.clone();

      self.eat();
      debug!("");
      debug!("right_node: (");
      let right_node = self.term();
      debug!(") // right_node");
      debug!("");

      node = match op {
        Some(Token::Plus) => self.plus(node, right_node),
        Some(Token::Minus) => self.minus(node, right_node),
        _ => panic!(),
      };

      debug!("");
      debug!("node: {:?}", node);
      debug!("");
    }
    
    node 
  }
}

pub fn parse(input: &str) -> Expr {
  let mut parser = Parser::new(input.to_string());

  match parser.expr() {
    Some(e) => e,
    None => panic!(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use expr::{Expr, BinOp};
  extern crate env_logger;

  #[test]
  fn test_mult_div() {
    //let _ = env_logger::init();
    assert_eq!(
      Expr::BinOp(
        BinOp::Times,
        Box::new(Expr::Int(3)),
        Box::new(Expr::Int(4)),
      ),
      parse("3*4")
    );

    assert_eq!(
      Expr::BinOp(
        BinOp::Div,
        Box::new(Expr::Int(3)),
        Box::new(Expr::Int(4)),
      ),
      parse("3/4")
    );
  }

  #[test]
  fn test_parse_add_subtract_parens() {
    //let _ = env_logger::init();
    assert_eq!(
      Expr::BinOp(
        BinOp::Plus,
        Box::new(Expr::Int(3)),
        Box::new(Expr::Int(4)),
      ),
      parse("3+4")
    );

    assert_eq!(
      Expr::BinOp(
        BinOp::Plus,
        Box::new(
          Expr::BinOp(
            BinOp::Plus,
            Box::new(Expr::Int(3)),
            Box::new(Expr::Int(4)),
          ),
        ),
        Box::new(Expr::Int(5)),
      ),
      parse("3+4+5")
    );

    assert_eq!(
      Expr::BinOp(BinOp::Plus,
        Box::new(Expr::Int(3)),
        Box::new(
          Expr::BinOp(
            BinOp::Plus,
            Box::new(Expr::Int(4)),
            Box::new(Expr::Int(5)),
          ),
        ),
      ),
      parse("3+(4+5)")
    );

    assert_eq!(
      Expr::BinOp(
        BinOp::Minus,
        Box::new(Expr::Int(3)),
        Box::new(Expr::Int(4)),
      ),
      parse("3-4")
    );

    assert_eq!(
      Expr::BinOp(
        BinOp::Minus,
        Box::new(
          Expr::BinOp(
            BinOp::Minus,
            Box::new(Expr::Int(3)),
            Box::new(Expr::Int(4)),
          ),
        ),
        Box::new(Expr::Int(5)),
      ),
      parse("3-4-5")
    );

    assert_eq!(
      Expr::BinOp(
        BinOp::Minus,
        Box::new(Expr::Int(3)),
        Box::new(
          Expr::BinOp(
            BinOp::Minus,
            Box::new(Expr::Int(4)),
            Box::new(Expr::Int(5)),
          ),
        ),
      ),
      parse("3-(4-5)")
    );

    assert_eq!(
      Expr::BinOp(
        BinOp::Minus,
        Box::new(
          Expr::BinOp(
            BinOp::Plus,
            Box::new(Expr::Int(4)),
            Box::new(Expr::Int(7)),
          ),
        ),
        Box::new(Expr::Int(3)),
      ),
      parse("(4+7)-3")
    );
  }
}
