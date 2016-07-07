#[derive(Debug, PartialEq)] 
pub enum BinOp {
  Plus,
  Minus,
  Times,
  Div,
}

#[derive(Debug, PartialEq)] 
pub enum Expr {
  Int(isize),
  BinOp(BinOp, Box<Expr>, Box<Expr>),
}

impl Expr {
  fn to_string(&self) -> String {
    match *self {
      Expr::Int(n) => {
        n.to_string()
      }
      _ => panic!("refusing to print a non-value")
    }
  }
}

pub fn eval(e: Expr) -> Expr {
  match e {
    Expr::BinOp(op, e1, e2) => {
      let v1 = eval(*e1);
      let v2 = eval(*e2);

      match (v1, v2) {
        (Expr::Int(n1), Expr::Int(n2)) => {
          match op {
            BinOp::Plus => Expr::Int(n1 + n2),
            BinOp::Minus => Expr::Int(n1 - n2),
            BinOp::Times => Expr::Int(n1 * n2),
            BinOp::Div => Expr::Int(n1 / n2),
          }
        },
        _ => panic!()
      }
    }
    Expr::Int(n) => Expr::Int(n),
  }
}
