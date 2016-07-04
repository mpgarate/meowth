use expr::{Expr, Op};

#[derive(Clone, Debug, PartialEq)] 
enum Token {
  Plus,
  Integer(i64),
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

    match int_str.parse::<i64>() {
      Ok(n) => {
        self.cut_input_by(int_str.len());
        return Some(Token::Integer(n));
      }
      Err(e) => panic!()
    }
  }

  pub fn get_next_token(&mut self) -> Option<Token> {
    match self.text.chars().next() {
      Some('+') => {
        self.cut_input_by(1);
        Some(Token::Plus)
      },
      Some(x) if x.is_numeric() => self.lex_integer(),
      None => None,
      _ => panic!()
    }
  }
}

struct Parser {
  lexer: Lexer,
}

impl Parser {
  pub fn new(text: String) -> Parser {
    Parser { lexer: Lexer::new(text) }
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
      },
      _ => {
        println!("invalid factor: {:?}", token);
        panic!();
      },
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
