#[derive(Debug, PartialEq)] 
pub enum Expr {
  Integer(i64),
  Plus(Box<Expr>, Box<Expr>),
  Minus(Box<Expr>, Box<Expr>),
  Times(Box<Expr>, Box<Expr>),
  Div(Box<Expr>, Box<Expr>),
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

// (4+7) - 3

pub fn eval(e: Expr) -> Expr {
  match e {
    Expr::Plus(e1, e2) => {
      let v1 = eval(*e1);
      let v2 = eval(*e2);
      match (v1, v2) {
        (Expr::Integer(n1), Expr::Integer(n2)) => {
          Expr::Integer(n1 + n2)
        }
        _ => panic!()
      }
    },
    Expr::Times(e1, e2) => {
      let v1 = eval(*e1);
      let v2 = eval(*e2);
      match (v1, v2) {
        (Expr::Integer(n1), Expr::Integer(n2)) => {
          Expr::Integer(n1 * n2)
        }
        _ => panic!()
      }
    },
    Expr::Div(e1, e2) => {
      let v1 = eval(*e1);
      let v2 = eval(*e2);
      match (v1, v2) {
        (Expr::Integer(n1), Expr::Integer(n2)) => {
          // TODO: don't store in an integer!
          Expr::Integer(n1 / n2)
        }
        _ => panic!()
      }
    },
    Expr::Minus(e1, e2) => {
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
