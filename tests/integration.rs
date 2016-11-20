extern crate meowth;

#[cfg(test)]
mod tests {
  extern crate meowth;
  use meowth::interpreter::Interpreter;
  use meowth::expr::Expr;
  use meowth::runtime_error::RuntimeError;

  extern crate env_logger;

  fn meowth(input: &str) -> Result<Expr, RuntimeError> {
    let mut interpreter = Interpreter::new();
    interpreter.eval(input)
  }

  #[test]
  pub fn test_parser_error() {
    let _ = env_logger::init();

    // TODO: consider expecting an error for this
    // assert_eq!(Ok(Expr::Int(2), meowth("1 +"));
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
      interpreter.eval("bike x = 3;").unwrap()
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
  pub fn test_pokedex_and_speak() {
    let _ = env_logger::init();

    // TODO: it would be nice to test the stdout for this
    assert_eq!(
      Ok(Expr::Undefined),
      meowth("
        bike x = 555;
        pokedex(x);
      ")
    );

    // TODO: it would be nice to test the stdout for this
    assert_eq!(
      Ok(Expr::Undefined),
      meowth("
        bike x = 555;
        speak(x);
      ")
    );

    assert_eq!(
      Err(RuntimeError::VariableNotFound(String::from("foo"))),
      meowth("
        speak(foo);
      ")
    );
  }

  #[test]
  pub fn test_comments() {
    let _ = env_logger::init();

    assert_eq!(
      Ok(Expr::Int(3)),
      meowth("
        bike i = 0;
        i = i + 2; // adding two
        i = i + 1; // adding one
        i
      ")
    );

    /*
     * TODO: block comments
    assert_eq!(
      Ok(Expr::Int(2)),
      meowth("
        bike i = 0;
        i = i + 2; 
        /* not doing this i = i + 1; */
        i
      ")
    );
    */
  }

  #[test]
  pub fn test_defend_loop() {
    let _ = env_logger::init();

    assert_eq!(
      Ok(Expr::Int(12)),
      meowth("
        bike i = 0;

        defend (i < 10) {
         if (i % 2 draws 0) {
           i = i + 1
         } run {
           i = i + 3
         }
        };
        i
      ")
    );

    assert_eq!(Ok(Expr::Int(11)), meowth("bike i = 1; defend (i < 11) { i = i + 1; i }; i"));
    assert_eq!(Ok(Expr::Int(10)), meowth("bike i = 1; bike x = 4; defend (i % 2 != 0) { i = i + x; x = x + 1; x }; i"));
    assert_eq!(
      Ok(Expr::Int(96)),
      meowth("
        attack foo(x) { x * 2 };
        bike x = 3;
        defend (foo(x) < 100) {
          x = foo(x)
        };
        x
      ")
    );

    assert_eq!(
      Ok(Expr::Int(96)),
      meowth("
        attack foo(x) { x * 2 };
        bike x = 3;
        defend ((x = foo(x)) < 96) { 0 };
        x
      ")
    );

    assert_eq!(
      Ok(Expr::Int(16)),
      meowth("
        attack foo(x) { x + 1 };
        bike x = 1;
        defend (x < 10) {
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

    assert_eq!(Ok(Expr::Undefined), meowth("bike x = 2;"));

    assert_eq!(
      Ok(Expr::Int(8)),
      meowth("
        bike x = 4;
        bike foo = attack(z) {
          x = z + 2;
        };
        foo(x);
        foo(x);
        x
      ")
    );
  }

  #[test]
  pub fn test_mut_bike() {
    let _ = env_logger::init();

    assert_eq!(
      Ok(Expr::Int(100)),
      meowth("
       bike x = 100;
       x = 10;
       give(x);
       x
     ")
    );

    assert_eq!(
      Ok(Expr::Int(10)),
      meowth("
       bike x = 100;
       x = 10;
       give(x)
     ")
    );

    assert_eq!(
      Err(RuntimeError::EmptyBike("x".to_string())),
      meowth("
       bike x = 100;
       give(x);
       give(x)
     ")
    );

    assert_eq!(Ok(Expr::Int(555)), meowth("bike x = 55; bike y = 500; x + y"));

    assert_eq!(
      Ok(Expr::Int(2)),
      meowth("bike x = 1; bike y = 2; x = y; y = 3; x")
    );

    assert_eq!(Ok(Expr::Int(2)), meowth("bike x = 1; x = 2; x"));

    assert_eq!(
      Ok(Expr::Int(5)),
      meowth("bike x = 3; bike y = 2; x = y; y = x; pokeball z = 1; z + x + y")
    );

    assert_eq!(
      Ok(Expr::Int(20)),
      meowth("
        bike x = 4;
        attack foo(z) {
          x * z 
        };
        foo(x) + x
      ")
    );

    assert_eq!(
      Ok(Expr::Int(15)),
      meowth("bike x = 4; attack foo(z) { bike x = 7; x + z }; foo(x) + x")
    );

    /*
       TODO: allow bike bindings so that attack params can be reassigned
    assert_eq!(
      Ok(Expr::Int(23)),
      meowth("bike x = 4; attack foo(z) { bike x = 7; z = x; x = 12; x + z }; foo(x) + x")
    );
    */

    assert_eq!(Ok(Expr::Int(2)), meowth("bike i = 1; i = i + 1; i"));

    assert_eq!(
      Ok(Expr::Int(13)),
      meowth("bike x = 10; bike foo = attack(x) { bike foo = attack (y) { bike x = 3; y + x }; foo(x) }; foo(x) ")
    );

    assert_eq!(Ok(Expr::Int(5)), meowth("bike x = 3; x = attack() { 4 + 1 }; x()"));
    assert_eq!(Ok(Expr::Int(3)), meowth("bike x = attack() { 4 + 1 }; x = 3; x"));
  }


  #[test]
  pub fn test_battle_rebattle_run() {
    let _ = env_logger::init();

    assert_eq!(
      Ok(Expr::Int(999)),
      meowth("bike b = 1; if (win) { b = 999; }; b ")
    );

    assert_eq!(
      Ok(Expr::Int(34)),
      meowth("if (win && lose) { 32 } rebattle (!win && win) { 33 } run { 34 }")
    );

    
    assert_eq!(
      Ok(Expr::Int(32)),
      meowth("if (win || lose) { 32 } rebattle (!win && win) { 33 } run { 34 }")
    );

    assert_eq!(
      Ok(Expr::Int(30)),
      meowth("if (win && lose) { 32 } run { 30 }")
    );

    assert_eq!(
      Ok(Expr::Int(52)),
      meowth("if (pokeball x = 4; x beats 3) { 52 } run { 30 }")
    );

    assert_eq!(
      Ok(Expr::Int(22)),
      meowth("if (win) { 11 } run { 0 }; 22")
    );
  }

  #[test]
  pub fn test_func() {
    let _ = env_logger::init();

    assert_eq!(
      Ok(Expr::Int(8)),
      meowth("
        bike x = 4;
        attack foo(z) {
          x = z + 2;
        };
        foo(x);
        foo(x);
        x
      ")
    );

    assert_eq!(Ok(Expr::Int(2)), meowth("pokeball x = 4; attack foo() { pokeball x = 1; x + 1 }; foo()"));
    assert_eq!(Ok(Expr::Int(6)), meowth("pokeball x = 5; attack foo() { x + 1 }; foo()"));
    assert_eq!(Ok(Expr::Int(60)), meowth("attack foo() { 5 }; attack bar() { attack foo() { 6 }; foo() * 10 }; bar()"));
    assert_eq!(Ok(Expr::Int(50)), meowth("attack foo() { 5 }; attack bar() { foo() * 10 }; bar()"));

    assert_eq!(Ok(Expr::Int(12)), meowth("attack sum(a, b) { a + b }; sum(sum(3, 4), 5)"));
    assert_eq!(Ok(Expr::Int(12)), meowth("attack tx_two(a) { 2 * a }; tx_two(tx_two(3))"));

    assert_eq!(
      Ok(Expr::Int(41)),
      meowth("
        attack foo(a) {
          a < 40 ? foo(a + 3) : a
        };

        foo(20)
      ")
    );

    assert_eq!(
      Ok(Expr::Int(21)),
      meowth("
        attack fib(n) {
          n draws 0 ? 0 : (n draws 1 ? 1 : fib(n - 1) + fib(n - 2))
        };

        fib(8)
      ")
    );

    assert_eq!(
      Ok(Expr::Int(21)),
      meowth("
        bike fib = attack(n) {
          n draws 0 ? 0 : (n draws 1 ? 1 : fib(n - 1) + fib(n - 2))
        };

        fib(8)
      ")
    );

    assert_eq!(
      Ok(Expr::Int(28)),
      meowth("
        attack foo(a) {
          1 + a
        };

        attack bar(b) {
          5 * b
        };
        
        foo(bar(4)) + 7
      ")
    );

    assert_eq!(Ok(Expr::Int(12)), meowth("attack b() { 5 + 5 }; pokeball a = b; a() + 2"));
    assert_eq!(Ok(Expr::Int(12)), meowth("pokeball b = attack() { 5 + 5 }; pokeball a = b; a() + 2"));
    assert_eq!(Ok(Expr::Int(12)), meowth("attack foo(a) { 1 + a }; foo(4) + 7"));
    assert_eq!(Ok(Expr::Int(12)), meowth("pokeball foo = attack(a) { 1 + a }; foo(4) + 7"));

    assert_eq!(Ok(Expr::Int(2)), meowth("attack foo() { 1 + 1 }; foo()"));
    assert_eq!(Ok(Expr::Int(7)), meowth("attack foo() { 1 + 3 }; foo() + 3"));
    assert_eq!(Ok(Expr::Int(9)), meowth("attack foo() { 1 + 3 }; attack bar() { foo() + 1}; 4 + bar()"));

    assert_eq!(Ok(Expr::Int(2)), meowth("pokeball foo = attack() { 1 + 1 }; foo()"));
    assert_eq!(Ok(Expr::Int(7)), meowth("pokeball foo = attack() { 1 + 3 }; foo() + 3"));
    assert_eq!(Ok(Expr::Int(9)), meowth("pokeball foo = attack() { 1 + 3 }; pokeball bar = attack() { foo() + 1}; 4 + bar()"));

    assert_eq!(Ok(Expr::Int(4)), meowth("attack() { 1 + 3 }()"));
    assert_eq!(Ok(Expr::Int(4)), meowth("pokeball foo = attack() { 1 + 3 }(); foo"));
  }

  #[test]
  pub fn test_const_decl() {
    let _ = env_logger::init();
    assert_eq!(Ok(Expr::Int(3)), meowth("pokeball x = 1 + 2; x"));
    assert_eq!(Ok(Expr::Int(1)), meowth("pokeball x = 1; x"));
    assert_eq!(Ok(Expr::Int(8)), meowth("pokeball x = 5; pokeball y = 3; pokeball z = x + y; z"));

    assert_eq!(Ok(Expr::Int(3)), meowth("pokeball x = (1 beats 2) ? 0 : 3; x"));

    // using pokeball keyword again re-binds value
    assert_eq!(Ok(Expr::Int(5)), meowth("pokeball x = 2; pokeball x = 3; x + 2"));

    assert_eq!(Ok(Expr::Int(52)), meowth("pokeball underscore_name = 51; 1 + underscore_name"));
  }

  #[test]
  pub fn test_ternary() {
    let _ = env_logger::init();
    assert_eq!(Ok(Expr::Int(1)), meowth("win ? 1 : 0"));
    assert_eq!(Ok(Expr::Int(0)), meowth("lose ? 1 : 0"));
    assert_eq!(Ok(Expr::Int(3)), meowth("(lose ? 1 : 0); 1 + 2"));
    assert_eq!(Ok(Expr::Int(3)), meowth("lose ? 1 : 0; 1 + 2"));
    assert_eq!(Ok(Expr::Int(0)), meowth("((1 + 1) beats 3) ? 1 : 0"));
    assert_eq!(Ok(Expr::Int(14)), meowth("((1 + 1) beats 3) ? win && lose : 12 + 2"));
    assert_eq!(Ok(Expr::Int(14)), meowth("1 + 1 beats 3 ? win && lose : 12 + 2"));
    assert_eq!(
      Ok(Expr::Int(10)),
      meowth(
          "(lose || win) ? ((1 + 2 beats 12) ? 9 : 10) : ((1 + 2 < 12) ? 6 : 7)"
       )
    );
    // same as above but without parens
    assert_eq!(
      Ok(Expr::Int(10)),
      meowth(
          "lose || win ? 1 + 2 beats 12 ? 9 : 10 : 1 + 2 < 12 ? 6 : 7"
       )
    );

    assert_eq!(Ok(Expr::Bool(true)), meowth("1 + 2 beats (1 draws 0 ? 5 : 1)"));

    assert_eq!(Ok(Expr::Int(-1)), meowth("win ;lose ? 1;2 : 0;-1"));
  }

  #[test]
  pub fn test_seq() {
    let _ = env_logger::init();
    assert_eq!(Ok(Expr::Int(5)), meowth("3;5"));
    assert_eq!(Ok(Expr::Int(4)), meowth("pokeball x = 3; pokeball y = 1;x + y"));
  }

  #[test]
  pub fn test_mod() {
    assert_eq!(Ok(Expr::Int(0)), meowth("1 % 1"));
    assert_eq!(Ok(Expr::Int(2)), meowth("7 % 5"));
    assert_eq!(Ok(Expr::Int(3)), meowth("-7 % 5"));
    assert_eq!(Ok(Expr::Int(-2)), meowth("-7 % -5"));
  }

  #[test]
  pub fn test_or_and_and() {
    assert_eq!(Ok(Expr::Bool(true)), meowth("win && win"));
    assert_eq!(Ok(Expr::Bool(false)), meowth("lose && lose"));
    assert_eq!(Ok(Expr::Bool(false)), meowth("win && lose"));
    assert_eq!(Ok(Expr::Bool(false)), meowth("lose && win"));

    assert_eq!(Ok(Expr::Bool(true)), meowth("win || win"));
    assert_eq!(Ok(Expr::Bool(false)), meowth("lose || lose"));
    assert_eq!(Ok(Expr::Bool(true)), meowth("win || lose"));
    assert_eq!(Ok(Expr::Bool(true)), meowth("lose || win"));
  }


  #[test]
  pub fn test_not_and_neg() {
    let _ = env_logger::init();

    assert_eq!(Ok(Expr::Bool(true)), meowth("!win || win"));

    assert_eq!(Ok(Expr::Int(0)), meowth("-1 * -1 + -1"));

    assert_eq!(Ok(Expr::Bool(true)), meowth("!lose"));

    assert_eq!(Ok(Expr::Bool(true)), meowth("!(win draws lose)"));
    assert_eq!(Ok(Expr::Bool(true)), meowth("!((1 draws 1) draws (3 <= 2))"));
    assert_eq!(Ok(Expr::Bool(false)), meowth("!((1 draws 1) draws !(3 <= 2))"));
    assert_eq!(Ok(Expr::Bool(true)), meowth("!!(!(!(win)))"));

    assert_eq!(Ok(Expr::Int(-1)), meowth("-1"));
    assert_eq!(Ok(Expr::Int(-100)), meowth("-(20 * 5)"));
    assert_eq!(Ok(Expr::Int(-100)), meowth("-(-20 * -5)"));
    assert_eq!(Ok(Expr::Int(-100)), meowth("(20 * -5)"));
    assert_eq!(Ok(Expr::Int(-100)), meowth("(-20 * 5)"));
    assert_eq!(Ok(Expr::Int(100)), meowth("(-20 * -5)"));
    assert_eq!(Ok(Expr::Int(100)), meowth("-(20 * -5)"));
    assert_eq!(Ok(Expr::Int(100)), meowth("-(-20 * 5)"));
    assert_eq!(Ok(Expr::Int(0)), meowth("1 + -1"));
    assert_eq!(Ok(Expr::Int(2)), meowth("1 - -1"));
    assert_eq!(Ok(Expr::Int(0)), meowth("-1 - -1"));
    assert_eq!(Ok(Expr::Int(-2)), meowth("-1 - 1"));
    assert_eq!(Ok(Expr::Int(-2)), meowth("-1 * 2"));
    assert_eq!(Ok(Expr::Int(-2)), meowth("2 * -1"));
    assert_eq!(Ok(Expr::Int(-2)), meowth("-2 * 1"));
    assert_eq!(Ok(Expr::Int(-1)), meowth("-(2 * 1) + 1"));
    assert_eq!(Ok(Expr::Int(1)), meowth("(2 * 1) + -1"));
  }

  #[test]
  pub fn test_comparison_operators() {
    assert_eq!(Ok(Expr::Bool(true)), meowth("1 draws 1"));
    assert_eq!(Ok(Expr::Bool(false)), meowth("1 draws 2"));
    assert_eq!(Ok(Expr::Bool(false)), meowth("(1 draws 1) draws (1 draws 2)"));
    assert_eq!(Ok(Expr::Bool(true)), meowth("(5 draws 2) draws (1 draws 2)"));
    assert_eq!(Ok(Expr::Bool(true)), meowth("(6 draws 6) draws win"));
    assert_eq!(Ok(Expr::Bool(false)), meowth("1 draws win"));
    assert_eq!(Ok(Expr::Bool(true)), meowth("lose draws lose"));

    assert_eq!(Ok(Expr::Bool(true)), meowth("1 beats 0"));
    assert_eq!(Ok(Expr::Bool(false)), meowth("1 < 0"));

    assert_eq!(Ok(Expr::Bool(true)), meowth("88 beats 34"));
    assert_eq!(Ok(Expr::Bool(false)), meowth("1 < 1"));
    assert_eq!(Ok(Expr::Bool(false)), meowth("1 beats 1"));

    assert_eq!(Ok(Expr::Bool(true)), meowth("88 != 34"));
    assert_eq!(Ok(Expr::Bool(false)), meowth("88 != 88"));
    assert_eq!(Ok(Expr::Bool(true)), meowth("88 <= 88"));
    assert_eq!(Ok(Expr::Bool(true)), meowth("88 survives 88"));
    assert_eq!(Ok(Expr::Bool(true)), meowth("1 survives 0"));
    assert_eq!(Ok(Expr::Bool(false)), meowth("1 survives 12"));

    assert_eq!(Ok(Expr::Bool(false)), meowth("win != win"));
    assert_eq!(Ok(Expr::Bool(true)), meowth("win != lose"));
  }

  #[test]
  pub fn test_spaces() {
    assert_eq!(Ok(Expr::Int(2)), meowth("1 + 1"));
    assert_eq!(Ok(Expr::Int(12)), meowth(" (3+   3)* 2      "));
    assert_eq!(Ok(Expr::Int(7)), meowth("1 + 3*(3 + (1 - 2))"));
  }

  #[test]
  pub fn test_eval_mult() {
    assert_eq!(Ok(Expr::Int(12)), meowth("6*2"));
    assert_eq!(Ok(Expr::Int(12)), meowth("(3+3)*2"));
    assert_eq!(Ok(Expr::Int(0)), meowth("(3+3)*0"));
  }

  #[test]
  pub fn test_eval_div() {
    assert_eq!(Ok(Expr::Int(6)), meowth("12/2"));
    //assert_eq!(Ok(Expr::Float(1.5)), meowth("3/2"));
  }

  #[test]
  pub fn test_eval_addition() {
    assert_eq!(Ok(Expr::Int(3)), meowth("1+2"));
    assert_eq!(Ok(Expr::Int(16)), meowth("5+7+4"));
    assert_eq!(Ok(Expr::Int(-1)), meowth("1-2"));
    assert_eq!(Ok(Expr::Int(-100)), meowth("32-132"));
    assert_eq!(Ok(Expr::Int(-120)), meowth("32-132-20"));

    assert_eq!(Ok(Expr::Int(-80)), meowth("32-(132-20)"));

    assert_eq!(Ok(Expr::Int(-6)), meowth("4-(7+3)"));
    assert_eq!(Ok(Expr::Int(0)), meowth("4-(7-3)"));
    assert_eq!(Ok(Expr::Int(8)), meowth("4+(7-3)"));
    assert_eq!(Ok(Expr::Int(8)), meowth("(4+7)-3)"));
    assert_eq!(Ok(Expr::Int(0)), meowth("(4-7)+3)"));
    assert_eq!(Ok(Expr::Int(14)), meowth("(4+7)+3)"));

    assert_eq!(Ok(Expr::Int(2)), meowth("(1-1)+(2-2)+(3-3)+((1+2)-((3-2)+1)+1)"));
    assert_eq!(Ok(Expr::Int(0)), meowth("((((((((((1-1)))+1))))-1)))"));
  }
}
