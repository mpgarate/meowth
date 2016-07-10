extern crate boxx;

use boxx::parser::{parse};
use boxx::expr::{eval};

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
  extern crate boxx;
  use boxx::parser::parse;
  use boxx::expr::{Expr, eval};
  extern crate env_logger;

  #[test]
  pub fn test_ternary() {
    let _ = env_logger::init();
    assert_eq!(Expr::Int(1), eval(parse("true ? 1 : 0")));
    assert_eq!(Expr::Int(0), eval(parse("false ? 1 : 0")));
    assert_eq!(Expr::Int(3), eval(parse("(false ? 1 : 0); 1 + 2")));
    assert_eq!(Expr::Int(3), eval(parse("false ? 1 : 0; 1 + 2")));
    assert_eq!(Expr::Int(0), eval(parse("((1 + 1) > 3) ? 1 : 0")));
    assert_eq!(Expr::Int(14), eval(parse("((1 + 1) > 3) ? true && false : 12 + 2")));
    assert_eq!(Expr::Int(14), eval(parse("1 + 1 > 3 ? true && false : 12 + 2")));
    assert_eq!(
      Expr::Int(10),
      eval(parse(
          "(false || true) ? ((1 + 2 > 12) ? 9 : 10) : ((1 + 2 < 12) ? 6 : 7)"
       ))
    );

    assert_eq!(Expr::Bool(true), eval(parse("1 + 2 > (1 == 0 ? 5 : 1)")));

    assert_eq!(Expr::Int(-1), eval(parse("true;false ? 1;2 : 0;-1")));
  }

  #[test]
  pub fn test_seq() {
    let _ = env_logger::init();
    assert_eq!(Expr::Int(5), eval(parse("3;5")));
    //assert_eq!(Expr::Int(4), eval(parse("let x = 3;let y = 1;x + y")));
  }

  #[test]
  pub fn test_mod() {
    assert_eq!(Expr::Int(0), eval(parse("1 % 1")));
    assert_eq!(Expr::Int(2), eval(parse("7 % 5")));
    assert_eq!(Expr::Int(3), eval(parse("-7 % 5")));
    assert_eq!(Expr::Int(-2), eval(parse("-7 % -5")));
  }

  #[test]
  pub fn test_or_and_and() {
    assert_eq!(Expr::Bool(true), eval(parse("true && true")));
    assert_eq!(Expr::Bool(false), eval(parse("false && false")));
    assert_eq!(Expr::Bool(false), eval(parse("true && false")));
    assert_eq!(Expr::Bool(false), eval(parse("false && true")));

    assert_eq!(Expr::Bool(true), eval(parse("true || true")));
    assert_eq!(Expr::Bool(false), eval(parse("false || false")));
    assert_eq!(Expr::Bool(true), eval(parse("true || false")));
    assert_eq!(Expr::Bool(true), eval(parse("false || true")));
  }


  #[test]
  pub fn test_not_and_neg() {
    assert_eq!(Expr::Bool(true), eval(parse("!false")));

    assert_eq!(Expr::Bool(true), eval(parse("!(true == false)")));
    assert_eq!(Expr::Bool(true), eval(parse("!((1 == 1) == (3 <= 2))")));
    assert_eq!(Expr::Bool(false), eval(parse("!((1 == 1) == !(3 <= 2))")));
    assert_eq!(Expr::Bool(true), eval(parse("!!(!(!(true)))")));

    assert_eq!(Expr::Int(-1), eval(parse("-1")));
    assert_eq!(Expr::Int(-100), eval(parse("-(20 * 5)")));
    assert_eq!(Expr::Int(-100), eval(parse("-(-20 * -5)")));
    assert_eq!(Expr::Int(-100), eval(parse("(20 * -5)")));
    assert_eq!(Expr::Int(-100), eval(parse("(-20 * 5)")));
    assert_eq!(Expr::Int(100), eval(parse("(-20 * -5)")));
    assert_eq!(Expr::Int(100), eval(parse("-(20 * -5)")));
    assert_eq!(Expr::Int(100), eval(parse("-(-20 * 5)")));
    assert_eq!(Expr::Int(0), eval(parse("1 + -1")));
    assert_eq!(Expr::Int(2), eval(parse("1 - -1")));
    assert_eq!(Expr::Int(0), eval(parse("-1 - -1")));
    assert_eq!(Expr::Int(-2), eval(parse("-1 - 1")));
    assert_eq!(Expr::Int(-2), eval(parse("-1 * 2")));
    assert_eq!(Expr::Int(-2), eval(parse("2 * -1")));
    assert_eq!(Expr::Int(-2), eval(parse("-2 * 1")));
    assert_eq!(Expr::Int(-1), eval(parse("-(2 * 1) + 1")));
    assert_eq!(Expr::Int(1), eval(parse("(2 * 1) + -1")));
  }

  #[test]
  pub fn test_comparison_operators() {
    assert_eq!(Expr::Bool(true), eval(parse("1 == 1")));
    assert_eq!(Expr::Bool(false), eval(parse("1 == 2")));
    assert_eq!(Expr::Bool(false), eval(parse("(1 == 1) == (1 == 2)")));
    assert_eq!(Expr::Bool(true), eval(parse("(5 == 2) == (1 == 2)")));
    assert_eq!(Expr::Bool(true), eval(parse("(6 == 6) == true")));
    assert_eq!(Expr::Bool(false), eval(parse("1 == true")));
    assert_eq!(Expr::Bool(true), eval(parse("false == false")));

    assert_eq!(Expr::Bool(true), eval(parse("1 > 0")));
    assert_eq!(Expr::Bool(false), eval(parse("1 < 0")));

    assert_eq!(Expr::Bool(true), eval(parse("88 > 34")));
    assert_eq!(Expr::Bool(false), eval(parse("1 < 1")));
    assert_eq!(Expr::Bool(false), eval(parse("1 > 1")));

    assert_eq!(Expr::Bool(true), eval(parse("88 != 34")));
    assert_eq!(Expr::Bool(false), eval(parse("88 != 88")));
    assert_eq!(Expr::Bool(true), eval(parse("88 <= 88")));
    assert_eq!(Expr::Bool(true), eval(parse("88 >= 88")));
    assert_eq!(Expr::Bool(true), eval(parse("1 >= 0")));
    assert_eq!(Expr::Bool(false), eval(parse("1 >= 12")));

    assert_eq!(Expr::Bool(false), eval(parse("true != true")));
    assert_eq!(Expr::Bool(true), eval(parse("true != false")));
  }

  #[test]
  pub fn test_spaces() {
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
