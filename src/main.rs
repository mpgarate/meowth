#[macro_use]
extern crate log;

mod parser;
mod expr;

use parser::parse;
use expr::{Expr, eval};

use std::io::{Write, stdout, stdin};

fn main() {
  loop {
    print!("calc> ");
    stdout().flush();

    let mut input = String::new();
    match stdin().read_line(&mut input) {
      Ok(bytes_read) => {
        if input == "exit\n".to_string() { 
          break;
        }
        println!("{:?}", eval(parse(&input)))
      },
      Err(e) => print!("error: {}", e)
    }
    stdout().flush();
  }
}

#[cfg(test)]
mod tests {
  use parser::parse;
  use expr::{Expr, eval};

  #[test]
  pub fn test_eval_mult() {
    assert_eq!(Expr::Integer(12), eval(parse("6*2")));
    assert_eq!(Expr::Integer(12), eval(parse("(3+3)*2")));
    assert_eq!(Expr::Integer(0), eval(parse("(3+3)*0")));
  }

  #[test]
  pub fn test_eval_div() {
    assert_eq!(Expr::Integer(6), eval(parse("12/2")));
    //assert_eq!(Expr::Float(1.5), eval(parse("3/2")));
  }

  #[test]
  pub fn test_eval_addition() {
    assert_eq!(Expr::Integer(3), eval(parse("1+2")));
    assert_eq!(Expr::Integer(16), eval(parse("5+7+4")));
    assert_eq!(Expr::Integer(-1), eval(parse("1-2")));
    assert_eq!(Expr::Integer(-100), eval(parse("32-132")));
    assert_eq!(Expr::Integer(-120), eval(parse("32-132-20")));

    assert_eq!(Expr::Integer(-80), eval(parse("32-(132-20)")));

    assert_eq!(Expr::Integer(-6), eval(parse("4-(7+3)")));
    assert_eq!(Expr::Integer(0), eval(parse("4-(7-3)")));
    assert_eq!(Expr::Integer(8), eval(parse("4+(7-3)")));
    assert_eq!(Expr::Integer(8), eval(parse("(4+7)-3)")));
    assert_eq!(Expr::Integer(0), eval(parse("(4-7)+3)")));
    assert_eq!(Expr::Integer(14), eval(parse("(4+7)+3)")));

    assert_eq!(Expr::Integer(2), eval(parse("(1-1)+(2-2)+(3-3)+((1+2)-((3-2)+1)+1)")));
    assert_eq!(Expr::Integer(0), eval(parse("((((((((((1-1)))+1))))-1)))")));
  }
}
