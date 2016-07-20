extern crate boxx;

#[cfg(test)]
mod tests {
  extern crate boxx;
  use boxx::expr::{Expr, boxx};
  extern crate env_logger;

  #[test]
  pub fn test_if_else() {
    let _ = env_logger::init();

    assert_eq!(
      Expr::Int(34),
      boxx("if (true && false) { 32 } else if (!true && true) { 33 } else { 34 }")
    );

    
    assert_eq!(
      Expr::Int(32),
      boxx("if (true || false) { 32 } else if (!true && true) { 33 } else { 34 }")
    );

    assert_eq!(
      Expr::Int(30),
      boxx("if (true && false) { 32 } else { 30 }")
    );

    assert_eq!(
      Expr::Int(52),
      boxx("if (let x = 4; x > 3) { 52 } else { 30 }")
    );

    assert_eq!(
      Expr::Int(22),
      boxx("if (true) { 11 } else { 0 }; 22")
    );
  }

  #[test]
  pub fn test_func() {
    let _ = env_logger::init();

    assert_eq!(Expr::Int(2), boxx("let x = 4; fn foo() { let x = 1; x + 1 }; foo()"));
    assert_eq!(Expr::Int(6), boxx("let x = 5; fn foo() { x + 1 }; foo()"));
    assert_eq!(Expr::Int(60), boxx("fn foo() { 5 }; fn bar() { fn foo() { 6 }; foo() * 10 }; bar()"));
    assert_eq!(Expr::Int(50), boxx("fn foo() { 5 }; fn bar() { foo() * 10 }; bar()"));

    assert_eq!(Expr::Int(12), boxx("fn sum(a, b) { a + b }; sum(sum(3, 4), 5)"));
    assert_eq!(Expr::Int(12), boxx("fn tx_two(a) { 2 * a }; tx_two(tx_two(3))"));

    assert_eq!(
      Expr::Int(41),
      boxx("
        fn foo(a) {
          a < 40 ? foo(a + 3) : a
        };

        foo(20)
      ")
    );

    assert_eq!(
      Expr::Int(21),
      boxx("
        fn fib(n) {
          n == 0 ? 0 : (n == 1 ? 1 : fib(n - 1) + fib(n - 2))
        };

        fib(8)
      ")
    );

    assert_eq!(
      Expr::Int(28),
      boxx("
        fn foo(a) {
          1 + a
        };

        fn bar(b) {
          5 * b
        };
        
        foo(bar(4)) + 7
      ")
    );

    assert_eq!(Expr::Int(12), boxx("fn foo(a) { 1 + a }; foo(4) + 7"));
    assert_eq!(Expr::Int(12), boxx("let foo = fn(a) { 1 + a }; foo(4) + 7"));

    assert_eq!(Expr::Int(2), boxx("fn foo() { 1 + 1 }; foo()"));
    assert_eq!(Expr::Int(7), boxx("fn foo() { 1 + 3 }; foo() + 3"));
    assert_eq!(Expr::Int(9), boxx("fn foo() { 1 + 3 }; fn bar() { foo() + 1}; 4 + bar()"));

    assert_eq!(Expr::Int(2), boxx("let foo = fn() { 1 + 1 }; foo()"));
    assert_eq!(Expr::Int(7), boxx("let foo = fn() { 1 + 3 }; foo() + 3"));
    assert_eq!(Expr::Int(9), boxx("let foo = fn() { 1 + 3 }; let bar = fn() { foo() + 1}; 4 + bar()"));
    // TODO: this should be parsed as a fn call assert_eq!(Expr::Int(4), boxx("fn() { 1 + 3 }()"));
    // TODO: have better failure message when ending a fn block with a semicolon
    // assert_eq!(Expr::Int(12), boxx("fn foo(a) { 1 + a; }; foo(4) + 7"));
  }

  #[test]
  pub fn test_const_decl() {
    let _ = env_logger::init();
    assert_eq!(Expr::Int(3), boxx("let x = 1 + 2; x"));
    assert_eq!(Expr::Int(1), boxx("let x = 1; x"));
    assert_eq!(Expr::Int(8), boxx("let x = 5; let y = 3; let z = x + y; z"));

    // using let keyword again re-binds value
    assert_eq!(Expr::Int(5), boxx("let x = 2; let x = 3; x + 2"));

    assert_eq!(Expr::Int(52), boxx("let underscore_name = 51; 1 + underscore_name"));
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
    assert_eq!(Expr::Int(4), boxx("let x = 3;let y = 1;x + y"));
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
    let _ = env_logger::init();

    // TODO: unop negate is not sticking to term
    assert_eq!(Expr::Bool(true), boxx("!true || true"));

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
