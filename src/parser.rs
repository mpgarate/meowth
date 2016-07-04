use expr::{Expr, Op};

#[derive(Clone, Debug, PartialEq)] 
enum Token {
  Plus,
  Integer(i64),
}

struct Lexer {
  text: String,
  pos: usize,
}

impl Lexer {
  pub fn new(text: String) -> Lexer {
    Lexer { text: text, pos: 0 }
  }

  fn remove_first(&mut self, n: usize) {
    let t = self.text.clone();
    let (old, remainder) = t.split_at(n);
    self.text = remainder.to_string();
  }

  pub fn integer(&mut self) -> Option<Token> {
    let (_, s) = self.text.split_at(self.pos);

    let int_str: String = s
      .chars()
      .take_while(|c| c.is_digit(10))
      .collect();

    match int_str.parse::<i64>() {
      Ok(n) => {
        self.pos += int_str.len();
        return Some(Token::Integer(n));
      }
      Err(e) => {
        println!("---------------{:?}", e);
        panic!();
      }
    }

    None
  }

  pub fn get_next_token(&mut self) -> Option<Token> {
    let old_pos = self.pos;

    match self.text.chars().nth(self.pos) {
      Some('+') => {
        self.pos += 1;
        Some(Token::Plus)
      },
      Some(x) if x.is_numeric() => self.integer(),
      None => None,
      _ => panic!()
    }
  }
}

struct Parser {
  text: String,
  lexer: Lexer,
}

impl Parser {
  pub fn new(text: String) -> Parser {
    let mut lexer = Lexer::new(text.clone());

    Parser {
      text: text,
      lexer: lexer,
    }
  }

  fn add(&mut self, e1: Option<Expr>, e2: Option<Expr>) -> Option<Expr> {
    match (e1, e2) {
      (Some(e1), Some(e2)) => {
        Some(
          Expr::BinOp(
            Op::Plus,
            Box::new(e1),
            Box::new(e2),
          )
        )
      },
      _ => panic!()
    }
  }

  fn factor(&mut self) -> Option<Expr> {
    let token = self.lexer.get_next_token();

    match token {
      Some(Token::Integer(n)) => {
        return Some(Expr::Integer(n));
      }
      Some(t) => {
        println!("invalid factor: {:?}", t);
        panic!();
      }
      _ => panic!()
    }
  }

  pub fn expr(&mut self) -> Option<Expr> {
    let mut node = self.factor();

    let mut token = self.lexer.get_next_token();

    while token == Some(Token::Plus) {
      let right_node = self.factor();
      node = self.add(node, right_node);

      token = self.lexer.get_next_token();
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

#[test]
fn test_parse_int() {
  assert_eq!(
    Expr::BinOp(
      Op::Plus,
        Box::new(Expr::Integer(3)),
        Box::new(Expr::Integer(4)),
      ),
    parse("3+4")
  );

  assert_eq!(
    Expr::BinOp(
      Op::Plus,
      Box::new(Expr::BinOp(
        Op::Plus,
        Box::new(Expr::Integer(3)),
        Box::new(Expr::Integer(4)),
        ),
      ),
      Box::new(Expr::Integer(5)),
    ),
    parse("3+4+5")
  );
}
