#[derive(Debug, PartialEq)] 
pub enum BinOp {
  Plus,
  Minus,
  Times,
  Div,
  Eq,
}

#[derive(Debug, PartialEq)] 
pub enum Expr {
  Int(isize),
  Bool(bool),
  BinOp(BinOp, Box<Expr>, Box<Expr>),
}

fn to_int(e: Expr) -> isize {
  match e {
    Expr::Int(n) => n,
    _ => {
      debug!("cant turn into int: {:?}", e);
      panic!()
    }
  }
}

pub fn eval(e: Expr) -> Expr {
  match e {
    Expr::BinOp(BinOp::Eq, e1, e2) => {
      Expr::Bool(eval(*e1) == eval(*e2))
    },
    Expr::BinOp(BinOp::Plus, e1, e2) => {
      Expr::Int(to_int(eval(*e1)) + to_int(eval(*e2)))
    },
    Expr::BinOp(BinOp::Minus, e1, e2) => {
      Expr::Int(to_int(eval(*e1)) - to_int(eval(*e2)))
    },
    Expr::BinOp(BinOp::Times, e1, e2) => {
      Expr::Int(to_int(eval(*e1)) * to_int(eval(*e2)))
    },
    Expr::BinOp(BinOp::Div, e1, e2) => {
      Expr::Int(to_int(eval(*e1)) / to_int(eval(*e2)))
    },
    Expr::Int(_) => e,
    Expr::Bool(_) => e,
  }
}
