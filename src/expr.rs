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
    Expr::BinOp(Op::Plus, v1, v2) => {
      match (*v1, *v2) {
        (Expr::Integer(n1), Expr::Integer(n2)) => {
          Expr::Integer(n1 + n2)
        }
        _ => panic!()
      }
    },
    Expr::BinOp(Op::Minus, v1, v2) => {
      match (*v1, *v2) {
        (Expr::Integer(n1), Expr::Integer(n2)) => {
          Expr::Integer(n1 - n2)
        }
        _ => panic!()
      }
    },
    Expr::Integer(n) => Expr::Integer(n),
  }
}
