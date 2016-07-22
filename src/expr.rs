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

fn is_int(e: &Expr) -> bool {
  match *e {
    Expr::Int(_) => true,
    _ => false,
  }
}

fn is_bool(e: &Expr) -> bool {
  match *e {
    Expr::Bool(_) => true,
    _ => false,
  }
}

fn is_func(e: &Expr) -> bool {
  match *e {
    Expr::Func(_, _, _) => true,
    _ => false,
  }
}

fn is_value(e: &Expr) -> bool {
  match *e {
    Expr::Int(_) | Expr::Bool(_) | Expr::Var(_) | Expr::Func(_, _, _) => true,
    _ => false,
  }
}

fn to_int(e: &Expr) -> isize {
  match *e {
    Expr::Int(n) => n,
    _ => {
      debug!("cant turn into int: {:?}", e);
      panic!()
    }
  }
}

fn to_bool(e: &Expr) -> bool {
  match *e {
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

pub fn step(e: Expr) -> Expr {
  debug!("step(e) : {:?}", e);
  match e {
    // TODO 'use Expr::*;
    //
    /**
     * Values are ineligible for step
     */
    Expr::Int(_) | Expr::Bool(_) | Expr::Func(_, _, _) | Expr::Var(_) => {
      debug!("stepping on a value {:?}", e);
      panic!("stepping on a value");
    }
    /**
     * Base cases
     */
    Expr::UnOp(UnOp::Not, ref e1) if is_bool(e1) => {
      Expr::Bool(!to_bool(&*e1))
    },
    Expr::UnOp(UnOp::Neg, ref e1) if is_int(e1) => {
      Expr::Int(-1 * to_int(&*e1))
    },
    Expr::BinOp(BinOp::And, ref e1, ref e2) if is_bool(e1) && is_bool(e2) => {
      Expr::Bool(to_bool(&*e1) && to_bool(&*e2))
    },
    Expr::BinOp(BinOp::Or, ref e1, ref e2) if is_bool(e1) && is_bool(e2) => {
      Expr::Bool(to_bool(&*e1) || to_bool(&*e2))
    },
    Expr::BinOp(BinOp::Eq, ref e1, ref e2) if is_value(e1) && is_value(e2) => {
      Expr::Bool(*e1 == *e2)
    },
    Expr::BinOp(BinOp::Ne, ref e1, ref e2) if is_value(e1) && is_value(e2) => {
      Expr::Bool(*e1 != *e2)
    },
    Expr::BinOp(BinOp::Mod, ref e1, ref e2) if is_int(e1) && is_int(e2) => {
      let n1 = to_int(&*e1);
      let n2 = to_int(&*e2);

      // rust % gives the remainder, not modulus
      let result = ((n1 % n2) + n2) % n2;

      Expr::Int(result)
    },
    Expr::BinOp(BinOp::Lt, ref e1, ref e2) if is_int(e1) && is_int(e2) => {
      Expr::Bool(to_int(&*e1) < to_int(&*e2))
    },
    Expr::BinOp(BinOp::Gt, ref e1, ref e2) if is_int(e1) && is_int(e2) => {
      Expr::Bool(to_int(&*e1) > to_int(&*e2))
    },
    Expr::BinOp(BinOp::Leq, ref e1, ref e2) if is_int(e1) && is_int(e2) => {
      Expr::Bool(to_int(&*e1) <= to_int(&*e2))
    },
    Expr::BinOp(BinOp::Geq, ref e1, ref e2) if is_int(e1) && is_int(e2) => {
      Expr::Bool(to_int(&*e1) >= to_int(&*e2))
    },
    Expr::BinOp(BinOp::Plus, ref e1, ref e2) if is_int(e1) && is_int(e2) => {
      Expr::Int(to_int(&*e1) + to_int(&*e2))
    },
    Expr::BinOp(BinOp::Minus, ref e1, ref e2) if is_int(e1) && is_int(e2) => {
      Expr::Int(to_int(&*e1) - to_int(&*e2))
    },
    Expr::BinOp(BinOp::Times, ref e1, ref e2) if is_int(e1) && is_int(e2) => {
      Expr::Int(to_int(&*e1) * to_int(&*e2))
    },
    Expr::BinOp(BinOp::Div, ref e1, ref e2) if is_int(e1) && is_int(e2) => {
      Expr::Int(to_int(&*e1) / to_int(&*e2))
    },
    Expr::BinOp(BinOp::Seq, ref e1, ref e2) if is_value(e1) => {
      *e2.clone()
    },
    Expr::Ternary(ref e1, ref e2, ref e3) if is_value(e1) => {
      match to_bool(&*e1) {
        true => *e2.clone(),
        false => *e3.clone(),
      }
    },
    Expr::Let(ref x, ref e1, ref e2) if is_value(e1) => {
      subst(*e2.clone(), *x.clone(), *e1.clone())
    },
    Expr::FnCall(ref v1, ref es) if is_func(v1) => {
      match **v1 {
        Expr::Func(ref name, ref e1, ref xs) => {
          // sub the params
          let exp = xs.iter().zip(es.iter())
            .fold(*e1.clone(), |exp, (xn, en)| subst(exp, xn.clone(), en.clone()));

          // sub the fn body for named functions
          match *name {
            None => exp,
            Some(ref s) => subst(exp, *s.clone(), *v1.clone())
          }
        },
        _ => {
          debug!("expected a Func, got {:?}", v1);
          panic!()
        },
      }
    },
    /**
     * Search Cases
     */
    Expr::BinOp(ref op, ref v1, ref e2) if is_value(v1) => {
      Expr::BinOp(op.clone(), Box::new(*v1.clone()), Box::new(step(*e2.clone())))
    },
    Expr::BinOp(op, e1, e2) => {
      Expr::BinOp(op, Box::new(step(*e1)), e2)
    },
    Expr::UnOp(op, e1) => {
      Expr::UnOp(op, Box::new(step(*e1)))
    },
    Expr::Ternary(e1, e2, e3) => {
      Expr::Ternary(Box::new(step(*e1)), e2, e3)
    },
    Expr::Let(ref e1, ref e2, ref e3) if is_value(e1) => {
      Expr::Let(Box::new(*e1.clone()), Box::new(step(*e2.clone())), Box::new(*e3.clone()))
    },
    Expr::Let(e1, e2, e3) => {
      Expr::Let(Box::new(step(*e1)), e2, e3)
    },
    Expr::FnCall(e1, mut xs) => {
      let mut found_first = true;

      for x in xs.iter_mut() {
        if is_value(x) && found_first == true {
          found_first = false;
          *x = step(x.clone());
        }
      }

      Expr::FnCall(e1, xs)
    }
  }
}

pub fn boxx(input: &str) -> Expr {
  let mut exp = parse(input);

  loop {
    if is_value(&exp) {
      return exp
    } else {
      exp = step(exp);
    }
  }
}
