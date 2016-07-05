#[derive(Debug, PartialEq)] 
pub enum Op {
  Plus,
  Minus,
}

#[derive(Debug, PartialEq)] 
pub enum Expr {
  Integer(i64),
  BinOp(Op, Box<Expr>, Box<Expr>),
}

impl Expr {
  fn to_string(&self) -> String {
    match *self {
      Expr::Integer(n) => {
        n.to_string()
      }
      _ => panic!("refusing to print a non-value")
    }
  }
}


fn eval(e: Expr) -> Expr {
  match e {
    Expr::BinOp(Op::Plus, e1, e2) => {
      let v1 = eval(*e1);
      let v2 = eval(*e2);
      match (v1, v2) {
        (Expr::Integer(n1), Expr::Integer(n2)) => {
          Expr::Integer(n1 + n2)
        }
        _ => panic!()
      }
    },
    Expr::BinOp(Op::Minus, e1, e2) => {
      let v1 = eval(*e1);
      let v2 = eval(*e2);
      match (v1, v2) {
        (Expr::Integer(n1), Expr::Integer(n2)) => {
          Expr::Integer(n1 - n2)
        }
        _ => panic!()
      }
    },
    Expr::Integer(n) => Expr::Integer(n),
  }
}

#[cfg(test)]
mod tests {
  use parser::parse;
  use super::{Expr, eval};

  #[test]
  pub fn test_eval_addition() {
    assert_eq!(Expr::Integer(3), eval(parse("1+2")));
    assert_eq!(Expr::Integer(16), eval(parse("5+7+4")));
    assert_eq!(Expr::Integer(-1), eval(parse("1-2")));
    assert_eq!(Expr::Integer(-100), eval(parse("32-132")));
    assert_eq!(Expr::Integer(-120), eval(parse("32-132-20")));
  }
}
