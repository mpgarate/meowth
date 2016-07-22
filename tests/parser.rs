extern crate boxx;

#[cfg(test)]
mod test {
  use boxx::parser::{parse};
  use boxx::expr::{Expr, BinOp};
  extern crate env_logger;

  #[test]
  fn test_mult_div() {
    assert_eq!(
      Expr::Bop(
        BinOp::Times,
        Box::new(Expr::Int(3)),
        Box::new(Expr::Int(4)),
      ),
      parse("3*4")
    );

    assert_eq!(
      Expr::Bop(
        BinOp::Div,
        Box::new(Expr::Int(3)),
        Box::new(Expr::Int(4)),
      ),
      parse("3/4")
    );
  }

  #[test]
  fn test_parse_add_subtract_parens() {
    assert_eq!(
      Expr::Bop(
        BinOp::Plus,
        Box::new(Expr::Int(3)),
        Box::new(Expr::Int(4)),
      ),
      parse("3+4")
    );

    assert_eq!(
      Expr::Bop(
        BinOp::Plus,
        Box::new(
          Expr::Bop(
            BinOp::Plus,
            Box::new(Expr::Int(3)),
            Box::new(Expr::Int(4)),
          ),
        ),
        Box::new(Expr::Int(5)),
      ),
      parse("3+4+5")
    );

    assert_eq!(
      Expr::Bop(BinOp::Plus,
        Box::new(Expr::Int(3)),
        Box::new(
          Expr::Bop(
            BinOp::Plus,
            Box::new(Expr::Int(4)),
            Box::new(Expr::Int(5)),
          ),
        ),
      ),
      parse("3+(4+5)")
    );

    assert_eq!(
      Expr::Bop(
        BinOp::Minus,
        Box::new(Expr::Int(3)),
        Box::new(Expr::Int(4)),
      ),
      parse("3-4")
    );

    assert_eq!(
      Expr::Bop(
        BinOp::Minus,
        Box::new(
          Expr::Bop(
            BinOp::Minus,
            Box::new(Expr::Int(3)),
            Box::new(Expr::Int(4)),
          ),
        ),
        Box::new(Expr::Int(5)),
      ),
      parse("3-4-5")
    );

    assert_eq!(
      Expr::Bop(
        BinOp::Minus,
        Box::new(Expr::Int(3)),
        Box::new(
          Expr::Bop(
            BinOp::Minus,
            Box::new(Expr::Int(4)),
            Box::new(Expr::Int(5)),
          ),
        ),
      ),
      parse("3-(4-5)")
    );

    assert_eq!(
      Expr::Bop(
        BinOp::Minus,
        Box::new(
          Expr::Bop(
            BinOp::Plus,
            Box::new(Expr::Int(4)),
            Box::new(Expr::Int(7)),
          ),
        ),
        Box::new(Expr::Int(3)),
      ),
      parse("(4+7)-3")
    );
  }
}
