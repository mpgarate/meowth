use parser::{parse};
use ast::Expr::*;
use ast::UnOp::*;
use ast::BinOp::*;
use ast::*;
use std::collections::HashMap;

fn subst(e: Expr, x: Expr, v: Expr) -> Expr {
  let sub = |e1: Expr| subst(e1.clone(), x.clone(), v.clone());

  match (e.clone(), x.clone()) {
    (Var(ref s1), Var(ref s2)) if s1 == s2 => v.clone(),
    (FnCall(v1, xs), _) => {
      let xs2 = xs.iter().map(|xn| sub(xn.clone())).collect();

      if *v1 == x {
        FnCall(Box::new(v.clone()), xs2)
      } else {
        FnCall(v1, xs2)
      }
    },
    (Var(_), _) => e,
    (Int(_), _) => e,
    (Bool(_), _) => e,
    (Bop(op, e1, e2), _) => { 
      Bop(
        op,
        Box::new(sub(*e1)),
        Box::new(sub(*e2))
      )
    },
    (Uop(op, e1), _) => Uop(op, Box::new(sub(*e1))),
    (Ternary(e1, e2, e3), _) => {
      Ternary(
        Box::new(sub(*e1)),
        Box::new(sub(*e2)),
        Box::new(sub(*e3))
      )
    },
    (Let(e1, e2, e3), _) => {
      let e3s = if *e1 == x {
        *e3
      } else {
        sub(*e3)
      };

      Let(
        Box::new(*e1),
        Box::new(sub(*e2)),
        Box::new(e3s)
      )
    },
    (Func(name, e1, xs), _) => {
      let xs2 = xs.iter().map(|xn| sub(xn.clone())).collect();
      Func(name, Box::new(sub(*e1)), xs2)
    },
  }
}

pub fn step(mut state: State) -> State {
  let st_step = |s: &mut State, e1: &Expr| step(s.with(e1.clone())).expr;

  //debug!("step(e) : {:?}", e);
  let e1 = match state.expr.clone() {
    /**
     * Values are ineligible for step
     */
    Int(_) | Bool(_) | Func(_, _, _) | Var(_) => {
      debug!("stepping on a value {:?}", state.expr);
      panic!("stepping on a value");
    },
    /**
     * Base cases
     */
    Uop(Not, ref e1) if is_bool(e1) => {
      Bool(!to_bool(e1))
    },
    Uop(Neg, ref e1) if is_int(e1) => {
      Int(-1 * to_int(e1))
    },
    Bop(And, ref e1, ref e2) if is_bool(e1) && is_bool(e2) => {
      Bool(to_bool(e1) && to_bool(e2))
    },
    Bop(Or, ref e1, ref e2) if is_bool(e1) && is_bool(e2) => {
      Bool(to_bool(e1) || to_bool(e2))
    },
    Bop(Eq, ref e1, ref e2) if is_value(e1) && is_value(e2) => {
      Bool(*e1 == *e2)
    },
    Bop(Ne, ref e1, ref e2) if is_value(e1) && is_value(e2) => {
      Bool(*e1 != *e2)
    },
    Bop(Mod, ref e1, ref e2) if is_int(e1) && is_int(e2) => {
      let n1 = to_int(e1);
      let n2 = to_int(e2);

      // rust % gives the remainder, not modulus
      let result = ((n1 % n2) + n2) % n2;

      Int(result)
    },
    Bop(Lt, ref e1, ref e2) if is_int(e1) && is_int(e2) => {
      Bool(to_int(e1) < to_int(e2))
    },
    Bop(Gt, ref e1, ref e2) if is_int(e1) && is_int(e2) => {
      Bool(to_int(e1) > to_int(e2))
    },
    Bop(Leq, ref e1, ref e2) if is_int(e1) && is_int(e2) => {
      Bool(to_int(e1) <= to_int(e2))
    },
    Bop(Geq, ref e1, ref e2) if is_int(e1) && is_int(e2) => {
      Bool(to_int(e1) >= to_int(e2))
    },
    Bop(Plus, ref e1, ref e2) if is_int(e1) && is_int(e2) => {
      Int(to_int(e1) + to_int(e2))
    },
    Bop(Minus, ref e1, ref e2) if is_int(e1) && is_int(e2) => {
      Int(to_int(e1) - to_int(e2))
    },
    Bop(Times, ref e1, ref e2) if is_int(e1) && is_int(e2) => {
      Int(to_int(e1) * to_int(e2))
    },
    Bop(Div, ref e1, ref e2) if is_int(e1) && is_int(e2) => {
      Int(to_int(e1) / to_int(e2))
    },
    Bop(Seq, ref e1, ref e2) if is_value(e1) => {
      *e2.clone()
    },
    Ternary(ref e1, ref e2, ref e3) if is_value(e1) => {
      match to_bool(e1) {
        true => *e2.clone(),
        false => *e3.clone(),
      }
    },
    Let(ref x, ref e1, ref e2) if is_value(e1) => {
      subst(*e2.clone(), *x.clone(), *e1.clone())
    },
    FnCall(ref v1, ref es) if is_func(v1) => {
      match **v1 {
        Func(ref name, ref e1, ref xs) => {
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
    Bop(ref op, ref v1, ref e2) if is_value(v1) => {
      Bop(
        op.clone(),
        Box::new(*v1.clone()),
        Box::new(st_step(&mut state, e2))
      )
    },
    Bop(op, e1, e2) => {
      Bop(op, Box::new(st_step(&mut state, &*e1)), e2)
    },
    Uop(op, e1) => {
      Uop(op, Box::new(st_step(&mut state, &*e1)))
    },
    Ternary(e1, e2, e3) => {
      Ternary(Box::new(st_step(&mut state, &*e1)), e2, e3)
    },
    Let(ref e1, ref e2, ref e3) if is_value(e1) => {
      Let(
        Box::new(*e1.clone()),
        Box::new(st_step(&mut state, e2)),
        Box::new(*e3.clone())
      )
    },
    Let(e1, e2, e3) => {
      Let(Box::new(st_step(&mut state, &*e1)), e2, e3)
    },
    FnCall(e1, mut xs) => {
      let mut found_first = true;

      for x in xs.iter_mut() {
        if is_value(x) && found_first == true {
          found_first = false;
          *x = st_step(&mut state, x);
        }
      }

      FnCall(e1, xs)
    }
  };

  state.with(e1)
}

pub fn boxx(input: &str) -> Expr {
  let mut state = State::from(parse(input));

  let mut num_iterations = 0;

  loop {
    num_iterations += 1;
    if is_value(&state.expr) {
      debug!("--- iterations: {}", num_iterations);
      return state.expr
    } else {
      state = step(state);
    }
  }
}
