extern crate boxx;

#[cfg(test)]
mod tests {
  extern crate boxx;
  use boxx::expr::{Expr, boxx};
  extern crate env_logger;

  #[test]
  pub fn test_func() {
    let _ = env_logger::init();
    assert_eq!(Expr::Int(2), boxx("let foo = fn() { 1 + 1 }; foo()"));
    assert_eq!(Expr::Int(7), boxx("let foo = fn() { 1 + 3 }; foo() + 3"));
    assert_eq!(Expr::Int(9), boxx("let foo = fn() { 1 + 3 }; let bar = fn() { foo() + 1}; 4 + bar()"));
    // TODO: this should be parsed as a fn call assert_eq!(Expr::Int(4), boxx("fn() { 1 + 3 }()"));
  }

  #[test]
  pub fn test_const_decl() {
    let _ = env_logger::init();
    assert_eq!(Expr::Int(3), boxx("let x = 1 + 2; x"));
    assert_eq!(Expr::Int(1), boxx("let x = 1; x"));
    assert_eq!(Expr::Int(8), boxx("let x = 5; let y = 3; let z = x + y; z"));
  }

  #[test]
  pub fn test_ternary() {
    let _ = env_logger::init();
    assert_eq!(Expr::Int(1), boxx("true ? 1 : 0"));
    assert_eq!(Expr::Int(0), boxx("false ? 1 : 0"));
    assert_eq!(Expr::Int(3), boxx("(false ? 1 : 0); 1 + 2"));
    assert_eq!(Expr::Int(3), boxx("false ? 1 : 0; 1 + 2"));
    assert_eq!(Expr::Int(0), boxx("((1 + 1) > 3) ? 1 : 0"));
    assert_eq!(Expr::Int(14), boxx("((1 + 1) > 3) ? true && false : 12 + 2"));
    assert_eq!(Expr::Int(14), boxx("1 + 1 > 3 ? true && false : 12 + 2"));
    assert_eq!(
      Expr::Int(10),
      boxx(
          "(false || true) ? ((1 + 2 > 12) ? 9 : 10) : ((1 + 2 < 12) ? 6 : 7)"
       )
    );
    // same as above but without parens
    assert_eq!(
      Expr::Int(10),
      boxx(
          "false || true ? 1 + 2 > 12 ? 9 : 10 : 1 + 2 < 12 ? 6 : 7"
       )
    );

    assert_eq!(Expr::Bool(true), boxx("1 + 2 > (1 == 0 ? 5 : 1)"));

    assert_eq!(Expr::Int(-1), boxx("true;false ? 1;2 : 0;-1"));
  }

  #[test]
  pub fn test_seq() {
    let _ = env_logger::init();
    assert_eq!(Expr::Int(5), boxx("3;5"));
    //assert_eq!(Expr::Int(4), boxx("let x = 3;let y = 1;x + y"));
  }

  #[test]
  pub fn test_mod() {
    assert_eq!(Expr::Int(0), boxx("1 % 1"));
    assert_eq!(Expr::Int(2), boxx("7 % 5"));
    assert_eq!(Expr::Int(3), boxx("-7 % 5"));
    assert_eq!(Expr::Int(-2), boxx("-7 % -5"));
  }

  #[test]
  pub fn test_or_and_and() {
    assert_eq!(Expr::Bool(true), boxx("true && true"));
    assert_eq!(Expr::Bool(false), boxx("false && false"));
    assert_eq!(Expr::Bool(false), boxx("true && false"));
    assert_eq!(Expr::Bool(false), boxx("false && true"));

    assert_eq!(Expr::Bool(true), boxx("true || true"));
    assert_eq!(Expr::Bool(false), boxx("false || false"));
    assert_eq!(Expr::Bool(true), boxx("true || false"));
    assert_eq!(Expr::Bool(true), boxx("false || true"));
  }


  #[test]
  pub fn test_not_and_neg() {
    assert_eq!(Expr::Bool(true), boxx("!false"));

    assert_eq!(Expr::Bool(true), boxx("!(true == false)"));
    assert_eq!(Expr::Bool(true), boxx("!((1 == 1) == (3 <= 2))"));
    assert_eq!(Expr::Bool(false), boxx("!((1 == 1) == !(3 <= 2))"));
    assert_eq!(Expr::Bool(true), boxx("!!(!(!(true)))"));

    assert_eq!(Expr::Int(-1), boxx("-1"));
    assert_eq!(Expr::Int(-100), boxx("-(20 * 5)"));
    assert_eq!(Expr::Int(-100), boxx("-(-20 * -5)"));
    assert_eq!(Expr::Int(-100), boxx("(20 * -5)"));
    assert_eq!(Expr::Int(-100), boxx("(-20 * 5)"));
    assert_eq!(Expr::Int(100), boxx("(-20 * -5)"));
    assert_eq!(Expr::Int(100), boxx("-(20 * -5)"));
    assert_eq!(Expr::Int(100), boxx("-(-20 * 5)"));
    assert_eq!(Expr::Int(0), boxx("1 + -1"));
    assert_eq!(Expr::Int(2), boxx("1 - -1"));
    assert_eq!(Expr::Int(0), boxx("-1 - -1"));
    assert_eq!(Expr::Int(-2), boxx("-1 - 1"));
    assert_eq!(Expr::Int(-2), boxx("-1 * 2"));
    assert_eq!(Expr::Int(-2), boxx("2 * -1"));
    assert_eq!(Expr::Int(-2), boxx("-2 * 1"));
    assert_eq!(Expr::Int(-1), boxx("-(2 * 1) + 1"));
    assert_eq!(Expr::Int(1), boxx("(2 * 1) + -1"));
  }

  #[test]
  pub fn test_comparison_operators() {
    assert_eq!(Expr::Bool(true), boxx("1 == 1"));
    assert_eq!(Expr::Bool(false), boxx("1 == 2"));
    assert_eq!(Expr::Bool(false), boxx("(1 == 1) == (1 == 2)"));
    assert_eq!(Expr::Bool(true), boxx("(5 == 2) == (1 == 2)"));
    assert_eq!(Expr::Bool(true), boxx("(6 == 6) == true"));
    assert_eq!(Expr::Bool(false), boxx("1 == true"));
    assert_eq!(Expr::Bool(true), boxx("false == false"));

    assert_eq!(Expr::Bool(true), boxx("1 > 0"));
    assert_eq!(Expr::Bool(false), boxx("1 < 0"));

    assert_eq!(Expr::Bool(true), boxx("88 > 34"));
    assert_eq!(Expr::Bool(false), boxx("1 < 1"));
    assert_eq!(Expr::Bool(false), boxx("1 > 1"));

    assert_eq!(Expr::Bool(true), boxx("88 != 34"));
    assert_eq!(Expr::Bool(false), boxx("88 != 88"));
    assert_eq!(Expr::Bool(true), boxx("88 <= 88"));
    assert_eq!(Expr::Bool(true), boxx("88 >= 88"));
    assert_eq!(Expr::Bool(true), boxx("1 >= 0"));
    assert_eq!(Expr::Bool(false), boxx("1 >= 12"));

    assert_eq!(Expr::Bool(false), boxx("true != true"));
    assert_eq!(Expr::Bool(true), boxx("true != false"));
  }

  #[test]
  pub fn test_spaces() {
    assert_eq!(Expr::Int(2), boxx("1 + 1"));
    assert_eq!(Expr::Int(12), boxx(" (3+   3)* 2      "));
    assert_eq!(Expr::Int(7), boxx("1 + 3*(3 + (1 - 2))"));
  }

  #[test]
  pub fn test_eval_mult() {
    assert_eq!(Expr::Int(12), boxx("6*2"));
    assert_eq!(Expr::Int(12), boxx("(3+3)*2"));
    assert_eq!(Expr::Int(0), boxx("(3+3)*0"));
  }

  #[test]
  pub fn test_eval_div() {
    assert_eq!(Expr::Int(6), boxx("12/2"));
    //assert_eq!(Expr::Float(1.5), boxx("3/2"));
  }

  #[test]
  pub fn test_eval_addition() {
    assert_eq!(Expr::Int(3), boxx("1+2"));
    assert_eq!(Expr::Int(16), boxx("5+7+4"));
    assert_eq!(Expr::Int(-1), boxx("1-2"));
    assert_eq!(Expr::Int(-100), boxx("32-132"));
    assert_eq!(Expr::Int(-120), boxx("32-132-20"));

    assert_eq!(Expr::Int(-80), boxx("32-(132-20)"));

    assert_eq!(Expr::Int(-6), boxx("4-(7+3)"));
    assert_eq!(Expr::Int(0), boxx("4-(7-3)"));
    assert_eq!(Expr::Int(8), boxx("4+(7-3)"));
    assert_eq!(Expr::Int(8), boxx("(4+7)-3)"));
    assert_eq!(Expr::Int(0), boxx("(4-7)+3)"));
    assert_eq!(Expr::Int(14), boxx("(4+7)+3)"));

    assert_eq!(Expr::Int(2), boxx("(1-1)+(2-2)+(3-3)+((1+2)-((3-2)+1)+1)"));
    assert_eq!(Expr::Int(0), boxx("((((((((((1-1)))+1))))-1)))"));
  }
}
