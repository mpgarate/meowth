#[macro_use]
extern crate log;

mod parser;
mod expr;

use parser::parse;
use expr::eval;

use std::io::{Write, stdout, stdin};

fn main() {
  loop {
    print!("boxx> ");
    let _ = stdout().flush();

    let mut input = String::new();
    match stdin().read_line(&mut input) {
      Ok(_) => {
        if input == "exit\n".to_string() { 
          break;
        }
        println!("{:?}", eval(parse(&input)))
      },
      Err(e) => print!("error: {}", e)
    }
    let _ = stdout().flush();
  }
}

#[cfg(test)]
mod tests {
  use parser::parse;
  use expr::{Expr, eval};
  extern crate env_logger;

  #[test]
  pub fn test_bool() {
    let _ = env_logger::init();
    assert_eq!(Expr::Bool(true), eval(parse("1 == 1")));
    assert_eq!(Expr::Bool(false), eval(parse("1 == 2")));
    assert_eq!(Expr::Bool(false), eval(parse("(1 == 1) == (1 == 2)")));
    assert_eq!(Expr::Bool(true), eval(parse("(5 == 2) == (1 == 2)")));
    assert_eq!(Expr::Bool(true), eval(parse("(6 == 6) == true")));
    assert_eq!(Expr::Bool(false), eval(parse("1 == true")));
    assert_eq!(Expr::Bool(true), eval(parse("false == false")));
  }

  #[test]
  pub fn test_spaces() {
    //let _ = env_logger::init();
    assert_eq!(Expr::Int(2), eval(parse("1 + 1")));
    assert_eq!(Expr::Int(12), eval(parse(" (3+   3)* 2      ")));
    assert_eq!(Expr::Int(7), eval(parse("1 + 3*(3 + (1 - 2))")));
  }

  #[test]
  pub fn test_eval_mult() {
    assert_eq!(Expr::Int(12), eval(parse("6*2")));
    assert_eq!(Expr::Int(12), eval(parse("(3+3)*2")));
    assert_eq!(Expr::Int(0), eval(parse("(3+3)*0")));
  }

  #[test]
  pub fn test_eval_div() {
    assert_eq!(Expr::Int(6), eval(parse("12/2")));
    //assert_eq!(Expr::Float(1.5), eval(parse("3/2")));
  }

  #[test]
  pub fn test_eval_addition() {
    assert_eq!(Expr::Int(3), eval(parse("1+2")));
    assert_eq!(Expr::Int(16), eval(parse("5+7+4")));
    assert_eq!(Expr::Int(-1), eval(parse("1-2")));
    assert_eq!(Expr::Int(-100), eval(parse("32-132")));
    assert_eq!(Expr::Int(-120), eval(parse("32-132-20")));

    assert_eq!(Expr::Int(-80), eval(parse("32-(132-20)")));

    assert_eq!(Expr::Int(-6), eval(parse("4-(7+3)")));
    assert_eq!(Expr::Int(0), eval(parse("4-(7-3)")));
    assert_eq!(Expr::Int(8), eval(parse("4+(7-3)")));
    assert_eq!(Expr::Int(8), eval(parse("(4+7)-3)")));
    assert_eq!(Expr::Int(0), eval(parse("(4-7)+3)")));
    assert_eq!(Expr::Int(14), eval(parse("(4+7)+3)")));

    assert_eq!(Expr::Int(2), eval(parse("(1-1)+(2-2)+(3-3)+((1+2)-((3-2)+1)+1)")));
    assert_eq!(Expr::Int(0), eval(parse("((((((((((1-1)))+1))))-1)))")));
  }
}
