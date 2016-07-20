use parser::{parse};

#[derive(Clone, Debug, PartialEq)] 
pub enum UnOp {
  Not,
  Neg,
}

#[derive(Clone, Debug, PartialEq)] 
pub enum BinOp {
  Plus,
  Minus,
  Times,
  Div,
  Eq,
  Ne,
  Leq,
  Geq,
  Lt,
  Gt,
  And,
  Or,
  Mod,
  Seq,
}

#[derive(Clone, Debug, PartialEq)] 
pub enum Expr {
  Int(isize),
  Bool(bool),
  Var(String),
  BinOp(BinOp, Box<Expr>, Box<Expr>),
  UnOp(UnOp, Box<Expr>),
  Ternary(Box<Expr>, Box<Expr>, Box<Expr>),
  Let(Box<Expr>, Box<Expr>, Box<Expr>),
  Func(Option<Box<Expr>>, Box<Expr>, Vec<Expr>),
  FnCall(Box<Expr>, Vec<Expr>),
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

fn to_bool(e: Expr) -> bool {
  match e {
    Expr::Bool(b) => b,
    _ => {
      debug!("cant turn into bool: {:?}", e);
      panic!()
    }
  }
}

fn subst(e: Expr, x: Expr, v: Expr) -> Expr {
  let sub = |e1: Expr| subst(e1.clone(), x.clone(), v.clone());

  match (e.clone(), x.clone()) {
    (Expr::Var(ref s1), Expr::Var(ref s2)) if s1 == s2 => v.clone(),
    (Expr::FnCall(v1, xs), _) => {
      let xs2 = xs.iter().map(|xn| sub(xn.clone())).collect();

      if *v1 == x {
        Expr::FnCall(Box::new(v.clone()), xs2)
      } else {
        Expr::FnCall(v1, xs2)
      }
    },
    (Expr::Var(_), _) => e,
    (Expr::Int(_), _) => e,
    (Expr::Bool(_), _) => e,
    (Expr::BinOp(op, e1, e2), _) => { 
      Expr::BinOp(
        op,
        Box::new(sub(*e1)),
        Box::new(sub(*e2))
      )
    },
    (Expr::UnOp(op, e1), _) => Expr::UnOp(op, Box::new(sub(*e1))),
    (Expr::Ternary(e1, e2, e3), _) => {
      Expr::Ternary(
        Box::new(sub(*e1)),
        Box::new(sub(*e2)),
        Box::new(sub(*e3))
      )
    },
    (Expr::Let(e1, e2, e3), _) => {
      let e3s = if *e1 == x {
        *e3
      } else {
        sub(*e3)
      };

      Expr::Let(
        Box::new(*e1),
        Box::new(sub(*e2)),
        Box::new(e3s)
      )
    },
    (Expr::Func(name, e1, xs), _) => {
      let xs2 = xs.iter().map(|xn| sub(xn.clone())).collect();
      Expr::Func(name, Box::new(sub(*e1)), xs2)
    },
  }
}

pub fn eval(e: Expr) -> Expr {
  match e {
    Expr::UnOp(UnOp::Not, e1) => {
      Expr::Bool(!to_bool(eval(*e1)))
    },
    Expr::UnOp(UnOp::Neg, e1) => {
      Expr::Int(-1 * to_int(eval(*e1)))
    },
    Expr::BinOp(BinOp::And, e1, e2) => {
      Expr::Bool(to_bool(eval(*e1)) && to_bool(eval(*e2)))
    },
    Expr::BinOp(BinOp::Or, e1, e2) => {
      Expr::Bool(to_bool(eval(*e1)) || to_bool(eval(*e2)))
    },
    Expr::BinOp(BinOp::Eq, e1, e2) => {
      Expr::Bool(eval(*e1) == eval(*e2))
    },
    Expr::BinOp(BinOp::Ne, e1, e2) => {
      Expr::Bool(eval(*e1) != eval(*e2))
    },
    Expr::BinOp(BinOp::Mod, e1, e2) => {
      let n1 = to_int(eval(*e1));
      let n2 = to_int(eval(*e2));

      // rust % gives the remainder, not modulus
      let result = ((n1 % n2) + n2) % n2;

      Expr::Int(result)
    },
    Expr::BinOp(BinOp::Lt, e1, e2) => {
      Expr::Bool(to_int(eval(*e1)) < to_int(eval(*e2)))
    },
    Expr::BinOp(BinOp::Gt, e1, e2) => {
      Expr::Bool(to_int(eval(*e1)) > to_int(eval(*e2)))
    },
    Expr::BinOp(BinOp::Leq, e1, e2) => {
      Expr::Bool(to_int(eval(*e1)) <= to_int(eval(*e2)))
    },
    Expr::BinOp(BinOp::Geq, e1, e2) => {
      Expr::Bool(to_int(eval(*e1)) >= to_int(eval(*e2)))
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
    Expr::BinOp(BinOp::Seq, e1, e2) => {
      eval(*e1);
      eval(*e2)
    },
    Expr::Ternary(e1, e2, e3) => {
      match to_bool(eval(*e1)) {
        true => eval(*e2),
        false => eval(*e3),
      }
    },
    Expr::Let(x, e1, e2) => {
      let v1 = eval(*e1);
      let me2 = subst(*e2, *x, v1);

      eval(me2)
    },
    Expr::FnCall(v1, es) => {
      match *v1 {
        Expr::Func(ref name, ref e1, ref xs) => {
          // sub the params
          let exp = xs.iter().zip(es.iter())
            .fold(
              *e1.clone(),
              |exp, (xn, en)| subst(exp, xn.clone(), en.clone())
            );

          // sub the fn body for named functions
          match *name {
            None => eval(exp),
            Some(ref s) => eval(subst(exp, *s.clone(), *v1.clone()))
          }
        },
        _ => {
          debug!("expected a Func, got {:?}", v1);
          panic!()
        },
      }
    },
    Expr::Var(_) => e,
    Expr::Int(_) => e,
    Expr::Bool(_) => e,
    Expr::Func(_, _, _) => e,
  }
}

pub fn boxx(input: &str) -> Expr {
  eval(parse(input))
}
