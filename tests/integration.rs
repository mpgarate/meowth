extern crate boxx;

#[cfg(test)]
mod tests {
  extern crate boxx;
  use boxx::interpreter::Interpreter;
  use boxx::expr::Expr;
  use boxx::runtime_error::RuntimeError;

  extern crate env_logger;

  fn boxx(input: &str) -> Result<Expr, RuntimeError> {
    let mut interpreter = Interpreter::new();
    interpreter.eval(input)
  }

  #[test]
  pub fn test_parser_error() {
    let _ = env_logger::init();

    // TODO: consider expecting an error for this
    // assert_eq!(Ok(Expr::Int(2), boxx("1 +"));
  }

  #[test]
  pub fn test_interpreter() {
    let _ = env_logger::init();

    let mut interpreter = Interpreter::new();

    assert_eq!(
      Expr::Int(2),
      interpreter.eval("1 + 1").unwrap()
    );

    assert_eq!(
      Expr::Undefined,
      interpreter.eval("var x = 3;").unwrap()
    );

    assert_eq!(
      Expr::Int(3),
      interpreter.eval("x").unwrap()
    );


    assert_eq!(
      Expr::Undefined,
      interpreter.eval("attack double(x) { x + x };").unwrap()
    );

    assert_eq!(
      Expr::Int(48),
      interpreter.eval("double(24)").unwrap()
    );
  }

  #[test]
  pub fn test_print() {
    let _ = env_logger::init();

    // TODO: it would be nice to test the stdout for this
    assert_eq!(
      Ok(Expr::Undefined),
      boxx("
        var x = 555;
        print(x);
      ")
    );
  }

  #[test]
  pub fn test_comments() {
    let _ = env_logger::init();

    assert_eq!(
      Ok(Expr::Int(3)),
      boxx("
        var i = 0;
        i = i + 2; // adding two
        i = i + 1; // adding one
        i
      ")
    );

    /*
     * TODO: block comments
    assert_eq!(
      Ok(Expr::Int(2)),
      boxx("
        var i = 0;
        i = i + 2; 
        /* not doing this i = i + 1; */
        i
      ")
    );
    */
  }

  #[test]
  pub fn test_while_loop() {
    let _ = env_logger::init();

    assert_eq!(
      Ok(Expr::Int(12)),
      boxx("
        var i = 0;

        while (i < 10) {
         if (i % 2 == 0) {
           i = i + 1
         } else {
           i = i + 3
         }
        };
        i
      ")
    );

    assert_eq!(Ok(Expr::Int(11)), boxx("var i = 1; while (i < 11) { i = i + 1; i }; i"));
    assert_eq!(Ok(Expr::Int(10)), boxx("var i = 1; var x = 4; while (i % 2 != 0) { i = i + x; x = x + 1; x }; i"));
    assert_eq!(
      Ok(Expr::Int(96)),
      boxx("
        attack foo(x) { x * 2 };
        var x = 3;
        while (foo(x) < 100) {
          x = foo(x)
        };
        x
      ")
    );

    assert_eq!(
      Ok(Expr::Int(96)),
      boxx("
        attack foo(x) { x * 2 };
        var x = 3;
        while ((x = foo(x)) < 96) { 0 };
        x
      ")
    );

    assert_eq!(
      Ok(Expr::Int(16)),
      boxx("
        attack foo(x) { x + 1 };
        var x = 1;
        while (x < 10) {
          x = foo(x);
          x = 2 * foo(x);
          x + 1
        };
        x
      ")
    );
  }

  #[test]
  pub fn test_undefined() {
    let _ = env_logger::init();

    assert_eq!(Ok(Expr::Undefined), boxx("var x = 2;"));

    assert_eq!(
      Ok(Expr::Int(8)),
      boxx("
        var x = 4;
        var foo = attack(z) {
          x = z + 2;
        };
        foo(x);
        foo(x);
        x
      ")
    );
  }

  #[test]
  pub fn test_mut_var() {
    let _ = env_logger::init();

    assert_eq!(Ok(Expr::Int(555)), boxx("var x = 55; var y = 500; x + y"));

    assert_eq!(
      Ok(Expr::Int(2)),
      boxx("var x = 1; var y = 2; x = y; y = 3; x")
    );

    assert_eq!(Ok(Expr::Int(2)), boxx("var x = 1; x = 2; x"));

    assert_eq!(
      Ok(Expr::Int(5)),
      boxx("var x = 3; var y = 2; x = y; y = x; let z = 1; z + x + y")
    );

    assert_eq!(
      Ok(Expr::Int(20)),
      boxx("
        var x = 4;
        attack foo(z) {
          x * z 
        };
        foo(x) + x
      ")
    );

    assert_eq!(
      Ok(Expr::Int(15)),
      boxx("var x = 4; attack foo(z) { var x = 7; x + z }; foo(x) + x")
    );

    /*
       TODO: allow var bindings so that attack params can be reassigned
    assert_eq!(
      Ok(Expr::Int(23)),
      boxx("var x = 4; attack foo(z) { var x = 7; z = x; x = 12; x + z }; foo(x) + x")
    );
    */

    assert_eq!(Ok(Expr::Int(2)), boxx("var i = 1; i = i + 1; i"));

    assert_eq!(
      Ok(Expr::Int(13)),
      boxx("var x = 10; var foo = attack(x) { var foo = attack (y) { var x = 3; y + x }; foo(x) }; foo(x) ")
    );

    assert_eq!(Ok(Expr::Int(5)), boxx("var x = 3; x = attack() { 4 + 1 }; x()"));
    assert_eq!(Ok(Expr::Int(3)), boxx("var x = attack() { 4 + 1 }; x = 3; x"));
  }


  #[test]
  pub fn test_if_else() {
    let _ = env_logger::init();

    assert_eq!(
      Ok(Expr::Int(999)),
      boxx("var b = 1; if (win) { b = 999; }; b ")
    );

    assert_eq!(
      Ok(Expr::Int(34)),
      boxx("if (win && lose) { 32 } else if (!win && win) { 33 } else { 34 }")
    );

    
    assert_eq!(
      Ok(Expr::Int(32)),
      boxx("if (win || lose) { 32 } else if (!win && win) { 33 } else { 34 }")
    );

    assert_eq!(
      Ok(Expr::Int(30)),
      boxx("if (win && lose) { 32 } else { 30 }")
    );

    assert_eq!(
      Ok(Expr::Int(52)),
      boxx("if (let x = 4; x beats 3) { 52 } else { 30 }")
    );

    assert_eq!(
      Ok(Expr::Int(22)),
      boxx("if (win) { 11 } else { 0 }; 22")
    );
  }

  #[test]
  pub fn test_func() {
    let _ = env_logger::init();

    assert_eq!(
      Ok(Expr::Int(8)),
      boxx("
        var x = 4;
        attack foo(z) {
          x = z + 2;
        };
        foo(x);
        foo(x);
        x
      ")
    );

    assert_eq!(Ok(Expr::Int(2)), boxx("let x = 4; attack foo() { let x = 1; x + 1 }; foo()"));
    assert_eq!(Ok(Expr::Int(6)), boxx("let x = 5; attack foo() { x + 1 }; foo()"));
    assert_eq!(Ok(Expr::Int(60)), boxx("attack foo() { 5 }; attack bar() { attack foo() { 6 }; foo() * 10 }; bar()"));
    assert_eq!(Ok(Expr::Int(50)), boxx("attack foo() { 5 }; attack bar() { foo() * 10 }; bar()"));

    assert_eq!(Ok(Expr::Int(12)), boxx("attack sum(a, b) { a + b }; sum(sum(3, 4), 5)"));
    assert_eq!(Ok(Expr::Int(12)), boxx("attack tx_two(a) { 2 * a }; tx_two(tx_two(3))"));

    assert_eq!(
      Ok(Expr::Int(41)),
      boxx("
        attack foo(a) {
          a < 40 ? foo(a + 3) : a
        };

        foo(20)
      ")
    );

    assert_eq!(
      Ok(Expr::Int(21)),
      boxx("
        attack fib(n) {
          n == 0 ? 0 : (n == 1 ? 1 : fib(n - 1) + fib(n - 2))
        };

        fib(8)
      ")
    );

    assert_eq!(
      Ok(Expr::Int(21)),
      boxx("
        var fib = attack(n) {
          n == 0 ? 0 : (n == 1 ? 1 : fib(n - 1) + fib(n - 2))
        };

        fib(8)
      ")
    );

    assert_eq!(
      Ok(Expr::Int(28)),
      boxx("
        attack foo(a) {
          1 + a
        };

        attack bar(b) {
          5 * b
        };
        
        foo(bar(4)) + 7
      ")
    );

    assert_eq!(Ok(Expr::Int(12)), boxx("attack b() { 5 + 5 }; let a = b; a() + 2"));
    assert_eq!(Ok(Expr::Int(12)), boxx("let b = attack() { 5 + 5 }; let a = b; a() + 2"));
    assert_eq!(Ok(Expr::Int(12)), boxx("attack foo(a) { 1 + a }; foo(4) + 7"));
    assert_eq!(Ok(Expr::Int(12)), boxx("let foo = attack(a) { 1 + a }; foo(4) + 7"));

    assert_eq!(Ok(Expr::Int(2)), boxx("attack foo() { 1 + 1 }; foo()"));
    assert_eq!(Ok(Expr::Int(7)), boxx("attack foo() { 1 + 3 }; foo() + 3"));
    assert_eq!(Ok(Expr::Int(9)), boxx("attack foo() { 1 + 3 }; attack bar() { foo() + 1}; 4 + bar()"));

    assert_eq!(Ok(Expr::Int(2)), boxx("let foo = attack() { 1 + 1 }; foo()"));
    assert_eq!(Ok(Expr::Int(7)), boxx("let foo = attack() { 1 + 3 }; foo() + 3"));
    assert_eq!(Ok(Expr::Int(9)), boxx("let foo = attack() { 1 + 3 }; let bar = attack() { foo() + 1}; 4 + bar()"));

    assert_eq!(Ok(Expr::Int(4)), boxx("attack() { 1 + 3 }()"));
    assert_eq!(Ok(Expr::Int(4)), boxx("let foo = attack() { 1 + 3 }(); foo"));
  }

  #[test]
  pub fn test_const_decl() {
    let _ = env_logger::init();
    assert_eq!(Ok(Expr::Int(3)), boxx("let x = 1 + 2; x"));
    assert_eq!(Ok(Expr::Int(1)), boxx("let x = 1; x"));
    assert_eq!(Ok(Expr::Int(8)), boxx("let x = 5; let y = 3; let z = x + y; z"));

    assert_eq!(Ok(Expr::Int(3)), boxx("let x = (1 beats 2) ? 0 : 3; x"));

    // using let keyword again re-binds value
    assert_eq!(Ok(Expr::Int(5)), boxx("let x = 2; let x = 3; x + 2"));

    assert_eq!(Ok(Expr::Int(52)), boxx("let underscore_name = 51; 1 + underscore_name"));
  }

  #[test]
  pub fn test_ternary() {
    let _ = env_logger::init();
    assert_eq!(Ok(Expr::Int(1)), boxx("win ? 1 : 0"));
    assert_eq!(Ok(Expr::Int(0)), boxx("lose ? 1 : 0"));
    assert_eq!(Ok(Expr::Int(3)), boxx("(lose ? 1 : 0); 1 + 2"));
    assert_eq!(Ok(Expr::Int(3)), boxx("lose ? 1 : 0; 1 + 2"));
    assert_eq!(Ok(Expr::Int(0)), boxx("((1 + 1) beats 3) ? 1 : 0"));
    assert_eq!(Ok(Expr::Int(14)), boxx("((1 + 1) beats 3) ? win && lose : 12 + 2"));
    assert_eq!(Ok(Expr::Int(14)), boxx("1 + 1 beats 3 ? win && lose : 12 + 2"));
    assert_eq!(
      Ok(Expr::Int(10)),
      boxx(
          "(lose || win) ? ((1 + 2 beats 12) ? 9 : 10) : ((1 + 2 < 12) ? 6 : 7)"
       )
    );
    // same as above but without parens
    assert_eq!(
      Ok(Expr::Int(10)),
      boxx(
          "lose || win ? 1 + 2 beats 12 ? 9 : 10 : 1 + 2 < 12 ? 6 : 7"
       )
    );

    assert_eq!(Ok(Expr::Bool(true)), boxx("1 + 2 beats (1 == 0 ? 5 : 1)"));

    assert_eq!(Ok(Expr::Int(-1)), boxx("win ;lose ? 1;2 : 0;-1"));
  }

  #[test]
  pub fn test_seq() {
    let _ = env_logger::init();
    assert_eq!(Ok(Expr::Int(5)), boxx("3;5"));
    assert_eq!(Ok(Expr::Int(4)), boxx("let x = 3;let y = 1;x + y"));
  }

  #[test]
  pub fn test_mod() {
    assert_eq!(Ok(Expr::Int(0)), boxx("1 % 1"));
    assert_eq!(Ok(Expr::Int(2)), boxx("7 % 5"));
    assert_eq!(Ok(Expr::Int(3)), boxx("-7 % 5"));
    assert_eq!(Ok(Expr::Int(-2)), boxx("-7 % -5"));
  }

  #[test]
  pub fn test_or_and_and() {
    assert_eq!(Ok(Expr::Bool(true)), boxx("win && win"));
    assert_eq!(Ok(Expr::Bool(false)), boxx("lose && lose"));
    assert_eq!(Ok(Expr::Bool(false)), boxx("win && lose"));
    assert_eq!(Ok(Expr::Bool(false)), boxx("lose && win"));

    assert_eq!(Ok(Expr::Bool(true)), boxx("win || win"));
    assert_eq!(Ok(Expr::Bool(false)), boxx("lose || lose"));
    assert_eq!(Ok(Expr::Bool(true)), boxx("win || lose"));
    assert_eq!(Ok(Expr::Bool(true)), boxx("lose || win"));
  }


  #[test]
  pub fn test_not_and_neg() {
    let _ = env_logger::init();

    assert_eq!(Ok(Expr::Bool(true)), boxx("!win || win"));

    assert_eq!(Ok(Expr::Int(0)), boxx("-1 * -1 + -1"));

    assert_eq!(Ok(Expr::Bool(true)), boxx("!lose"));

    assert_eq!(Ok(Expr::Bool(true)), boxx("!(win == lose)"));
    assert_eq!(Ok(Expr::Bool(true)), boxx("!((1 == 1) == (3 <= 2))"));
    assert_eq!(Ok(Expr::Bool(false)), boxx("!((1 == 1) == !(3 <= 2))"));
    assert_eq!(Ok(Expr::Bool(true)), boxx("!!(!(!(win)))"));

    assert_eq!(Ok(Expr::Int(-1)), boxx("-1"));
    assert_eq!(Ok(Expr::Int(-100)), boxx("-(20 * 5)"));
    assert_eq!(Ok(Expr::Int(-100)), boxx("-(-20 * -5)"));
    assert_eq!(Ok(Expr::Int(-100)), boxx("(20 * -5)"));
    assert_eq!(Ok(Expr::Int(-100)), boxx("(-20 * 5)"));
    assert_eq!(Ok(Expr::Int(100)), boxx("(-20 * -5)"));
    assert_eq!(Ok(Expr::Int(100)), boxx("-(20 * -5)"));
    assert_eq!(Ok(Expr::Int(100)), boxx("-(-20 * 5)"));
    assert_eq!(Ok(Expr::Int(0)), boxx("1 + -1"));
    assert_eq!(Ok(Expr::Int(2)), boxx("1 - -1"));
    assert_eq!(Ok(Expr::Int(0)), boxx("-1 - -1"));
    assert_eq!(Ok(Expr::Int(-2)), boxx("-1 - 1"));
    assert_eq!(Ok(Expr::Int(-2)), boxx("-1 * 2"));
    assert_eq!(Ok(Expr::Int(-2)), boxx("2 * -1"));
    assert_eq!(Ok(Expr::Int(-2)), boxx("-2 * 1"));
    assert_eq!(Ok(Expr::Int(-1)), boxx("-(2 * 1) + 1"));
    assert_eq!(Ok(Expr::Int(1)), boxx("(2 * 1) + -1"));
  }

  #[test]
  pub fn test_comparison_operators() {
    assert_eq!(Ok(Expr::Bool(true)), boxx("1 == 1"));
    assert_eq!(Ok(Expr::Bool(false)), boxx("1 == 2"));
    assert_eq!(Ok(Expr::Bool(false)), boxx("(1 == 1) == (1 == 2)"));
    assert_eq!(Ok(Expr::Bool(true)), boxx("(5 == 2) == (1 == 2)"));
    assert_eq!(Ok(Expr::Bool(true)), boxx("(6 == 6) == win"));
    assert_eq!(Ok(Expr::Bool(false)), boxx("1 == win"));
    assert_eq!(Ok(Expr::Bool(true)), boxx("lose == lose"));

    assert_eq!(Ok(Expr::Bool(true)), boxx("1 beats 0"));
    assert_eq!(Ok(Expr::Bool(false)), boxx("1 < 0"));

    assert_eq!(Ok(Expr::Bool(true)), boxx("88 beats 34"));
    assert_eq!(Ok(Expr::Bool(false)), boxx("1 < 1"));
    assert_eq!(Ok(Expr::Bool(false)), boxx("1 beats 1"));

    assert_eq!(Ok(Expr::Bool(true)), boxx("88 != 34"));
    assert_eq!(Ok(Expr::Bool(false)), boxx("88 != 88"));
    assert_eq!(Ok(Expr::Bool(true)), boxx("88 <= 88"));
    assert_eq!(Ok(Expr::Bool(true)), boxx("88 >= 88"));
    assert_eq!(Ok(Expr::Bool(true)), boxx("1 >= 0"));
    assert_eq!(Ok(Expr::Bool(false)), boxx("1 >= 12"));

    assert_eq!(Ok(Expr::Bool(false)), boxx("win != win"));
    assert_eq!(Ok(Expr::Bool(true)), boxx("win != lose"));
  }

  #[test]
  pub fn test_spaces() {
    assert_eq!(Ok(Expr::Int(2)), boxx("1 + 1"));
    assert_eq!(Ok(Expr::Int(12)), boxx(" (3+   3)* 2      "));
    assert_eq!(Ok(Expr::Int(7)), boxx("1 + 3*(3 + (1 - 2))"));
  }

  #[test]
  pub fn test_eval_mult() {
    assert_eq!(Ok(Expr::Int(12)), boxx("6*2"));
    assert_eq!(Ok(Expr::Int(12)), boxx("(3+3)*2"));
    assert_eq!(Ok(Expr::Int(0)), boxx("(3+3)*0"));
  }

  #[test]
  pub fn test_eval_div() {
    assert_eq!(Ok(Expr::Int(6)), boxx("12/2"));
    //assert_eq!(Ok(Expr::Float(1.5)), boxx("3/2"));
  }

  #[test]
  pub fn test_eval_addition() {
    assert_eq!(Ok(Expr::Int(3)), boxx("1+2"));
    assert_eq!(Ok(Expr::Int(16)), boxx("5+7+4"));
    assert_eq!(Ok(Expr::Int(-1)), boxx("1-2"));
    assert_eq!(Ok(Expr::Int(-100)), boxx("32-132"));
    assert_eq!(Ok(Expr::Int(-120)), boxx("32-132-20"));

    assert_eq!(Ok(Expr::Int(-80)), boxx("32-(132-20)"));

    assert_eq!(Ok(Expr::Int(-6)), boxx("4-(7+3)"));
    assert_eq!(Ok(Expr::Int(0)), boxx("4-(7-3)"));
    assert_eq!(Ok(Expr::Int(8)), boxx("4+(7-3)"));
    assert_eq!(Ok(Expr::Int(8)), boxx("(4+7)-3)"));
    assert_eq!(Ok(Expr::Int(0)), boxx("(4-7)+3)"));
    assert_eq!(Ok(Expr::Int(14)), boxx("(4+7)+3)"));

    assert_eq!(Ok(Expr::Int(2)), boxx("(1-1)+(2-2)+(3-3)+((1+2)-((3-2)+1)+1)"));
    assert_eq!(Ok(Expr::Int(0)), boxx("((((((((((1-1)))+1))))-1)))"));
  }
}
