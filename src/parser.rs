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
    // TODO: is the clone really necessary?
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

  /*
     assert_eq!(IResult::Done(&b""[..], 7), parse_int(b"7"));
     assert_eq!(IResult::Done(&b" + 5"[..], 7), parse_int(b"7 + 5"));
     assert_eq!(IResult::Done(b""[..], Expr::Integer(-1)), parse_int(b"-1"));

     assert_eq!(IResult::Done(b""[..], Expr::Integer(123)), parse_int(b"123"));
     assert_eq!(IResult::Done(b""[..], Expr::Integer(-100)), parse_int(b"-100"));

     assert_eq!(IResult::Done(b""[..], Expr::Integer(-1)), parse_int(b"-1d"));
     assert_eq!(IResult::Done(b""[..], Expr::Integer(-12)), parse_int(b"-12-"));

     assert!(parse_int(b"d").is_err());
     assert!(parse_int(b"-d").is_err());
     assert!(parse_int(b"--").is_err());
     */
}


// Parser definition


/*
   pub fn parse_sign(s: &str) -> Result<Expr, &str> {
   }
   */

/*

   use std::str;
   use std::str::FromStr;

   pub fn parse_int(s: &str) -> ParseResult {
   let mut pos = 0;

   for (i,c) in s.chars().enumerate() {
   if i == 0 && c == '-' {
   pos = i + 1;
   continue;
   } else if c.is_digit(10) {
   pos = i + 1;
   continue;
   } else {
   break;
   }
   }

   let err_msg = "invalid integer";

   if pos > 0 {
   if pos == 1 && s.starts_with('-') {
   return (Err(err_msg), str)
   } else {
   let (int_str, remainder) = s.split_at(pos);
   return (IResult::Done(Done(b""[..], Expr::Integer(int_str.parse::<i64>().unwrap())), str)
   }
   }

   (Err(err_msg), str)
   }
   */

/*

   use nom::{IResult,digit};
   use nom::IResult::*;

   named!(parse_int<i64>,
   map_res!(
   map_res!(
   digit,
   str::from_utf8
   ),
   FromStr::from_str
   )
   );

*/


  /*
  pub fn integer(&self) -> Option<Expr> {
    let int_str: String = self.text
      .chars()
      .take_while(|c| c.is_digit(10))
      .collect();

    match int_str.parse::<i64>() {
      Ok(n) => return Some(Expr::Integer(n)),
      Err(e) => {
        println!("{:?}", e);
        panic!();
      }
    }

    None
  }

  pub fn plus(&mut self) -> Option<Expr> {
    if self.text.starts_with('+') {
      let left = self.expr().unwrap();
      let right = self.expr().unwrap();

      return Some(
        Expr::BinOp(
          Op::Plus,
          Box::new(left),
          Box::new(right),
        )
      );
    } else {
      panic!()
    }
  }
  */
