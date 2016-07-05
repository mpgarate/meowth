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


pub fn eval(e: Expr) -> Expr {
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
