use expr::{Expr, Op};

#[derive(Clone, Debug, PartialEq)] 
enum Token {
  Plus,
  Integer(i64),
}

struct Lexer {
  text: String,
  pos: i64,
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
    let int_str: String = self.text
      .chars()
      .take_while(|c| c.is_digit(10))
      .collect();

    match int_str.parse::<i64>() {
      Ok(n) => {
        self.remove_first(int_str.len());
        return Some(Token::Integer(n));
      }
      Err(e) => {
        println!("---------------{:?}", e);
        panic!();
      }
    }

    None
  }

  pub fn plus(&mut self) -> Option<Token> {
    if self.text.starts_with('+') {
      self.remove_first(1);
      return Some(Token::Plus);
    } else {
      panic!();
    }
  }

  pub fn get_next_token(&mut self) -> Option<Token> {
    // TODO: make this do real things

    self.pos += 1;

    match self.pos {
      1 => self.integer(),
      2 => self.plus(),
      3 => self.integer(),
      4 => None,
      _ => {
        println!("----------{:?}", self.pos);
        panic!();
      }
    }
  }
}

struct Parser {
  text: String,
  lexer: Lexer,
  current_token: Option<Token>,
}

impl Parser {
  pub fn new(text: String) -> Parser {
    let mut lexer = Lexer::new(text.clone());

    let token = lexer.get_next_token();

    Parser {
      text: text,
      lexer: lexer,
      current_token: token,
    }
  }

  pub fn eat(&mut self) {
    self.current_token = self.lexer.get_next_token();
  }

  pub fn factor(&mut self) -> Option<Expr> {
    let token = self.current_token.clone();
    self.eat();

    match token {
      Some(Token::Integer(n)) => {
        return Some(Expr::Integer(n));
      }
      Some(t) => {
        println!("--------------------{:?}", t);
        panic!();
      }
      _ => panic!()
    }
  }

  pub fn expr(&mut self) -> Option<Expr> {
    let factor = self.factor();

    let token = self.current_token.clone();
    self.eat();

    match token {
      Some(Token::Plus) => {
        let right = self.current_token.clone();

        match (factor, right) {
          (
            Some(e),
            Some(Token::Integer(n2)),
          ) => Some(
            Expr::BinOp(
              Op::Plus,
              Box::new(e),
              Box::new(Expr::Integer(n2)),
            )
          ),
          (a, b) => {
            println!("token: {:?}", a);
            println!("right: {:?}", b);
            panic!();
          }
        }
      },
      _ => panic!()
    }
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
}
