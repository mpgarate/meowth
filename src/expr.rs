use parser::{parse};
use ast::Expr::*;
use ast::UnOp::*;
use ast::BinOp::*;
use ast::*;

fn subst(e: Expr, x: Expr, v: Expr) -> Expr {
  let sub = |e1: Expr| subst(e1.clone(), x.clone(), v.clone());

  match (e.clone(), x.clone()) {
    (Var(ref s1), Var(ref s2)) if s1 == s2 => v.clone(),
    (Addr(ref a), Addr(ref b)) if a == b => v.clone(),
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
    (Addr(_), _) => e,
    (Bop(op, e1, e2), _) => { 
      Bop(
        op,
        Box::new(sub(*e1)),
        Box::new(sub(*e2))
      )
    },
    (Assign(e1, e2, e3), _) => {
      Assign(
        Box::new(sub(*e1)),
        Box::new(sub(*e2)),
        Box::new(sub(*e3))
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
    (Decl(d, e1, e2, e3), _) => {
      Decl(
        d,
        Box::new(*e1),
        Box::new(sub(*e2)),
        Box::new(sub(*e3))
      )
    }
  }
}

pub fn step(mut state: State) -> State {
  let st_step = |s: &mut State, e1: &Expr| step(s.with(e1.clone())).expr;

  debug!("step(e) : {:?}", state.expr);
  debug!("step(state) : {:?}", state.mem);
  let e1 = match state.expr.clone() {
    Addr(addr) => {
      state.get(addr)
    },
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
    Uop(Not, ref e1) if e1.is_bool() => {
      Bool(!e1.to_bool())
    },
    Uop(Neg, ref e1) if e1.is_int() => {
      Int(-1 * e1.to_int())
    },
    Bop(And, ref e1, ref e2) if e1.is_bool() && e2.is_bool() => {
      Bool(e1.to_bool() && e2.to_bool())
    },
    Bop(Or, ref e1, ref e2) if e1.is_bool() && e2.is_bool() => {
      Bool(e1.to_bool() || e2.to_bool())
    },
    Bop(Eq, ref e1, ref e2) if e1.is_value() && e2.is_value() => {
      Bool(*e1 == *e2)
    },
    Bop(Ne, ref e1, ref e2) if e1.is_value() && e2.is_value() => {
      Bool(*e1 != *e2)
    },
    Bop(Mod, ref e1, ref e2) if e1.is_int() && e2.is_int() => {
      let n1 = e1.to_int();
      let n2 = e2.to_int();

      // rust % gives the remainder, not modulus
      let result = ((n1 % n2) + n2) % n2;

      Int(result)
    },
    Bop(Lt, ref e1, ref e2) if e1.is_int() && e2.is_int() => {
      Bool(e1.to_int() < e2.to_int())
    },
    Bop(Gt, ref e1, ref e2) if e1.is_int() && e2.is_int() => {
      Bool(e1.to_int() > e2.to_int())
    },
    Bop(Leq, ref e1, ref e2) if e1.is_int() && e2.is_int() => {
      Bool(e1.to_int() <= e2.to_int())
    },
    Bop(Geq, ref e1, ref e2) if e1.is_int() && e2.is_int() => {
      Bool(e1.to_int() >= e2.to_int())
    },
    Bop(Plus, ref e1, ref e2) if e1.is_int() && e2.is_int() => {
      Int(e1.to_int() + e2.to_int())
    },
    Bop(Minus, ref e1, ref e2) if e1.is_int() && e2.is_int() => {
      Int(e1.to_int() - e2.to_int())
    },
    Bop(Times, ref e1, ref e2) if e1.is_int() && e2.is_int() => {
      Int(e1.to_int() * e2.to_int())
    },
    Bop(Div, ref e1, ref e2) if e1.is_int() && e2.is_int() => {
      Int(e1.to_int() / e2.to_int())
    },
    Bop(Seq, ref e1, ref e2) if e1.is_value() => {
      *e2.clone()
    },
    Assign(ref v1, ref v2, ref e3) if v1.is_addr() && v2.is_value() => {
      let addr = v1.to_addr();
      state.assign(addr.clone(), *v2.clone());
      debug!("done assigning {:?}", state.mem);
      *e3.clone()
    },
    Ternary(ref e1, ref e2, ref e3) if e1.is_value() => {
      match e1.to_bool() {
        true => *e2.clone(),
        false => *e3.clone(),
      }
    },
    Let(ref x, ref e1, ref e2) if e1.is_value() => {
      subst(*e2.clone(), *x.clone(), *e1.clone())
    },
    Decl(ref dt, ref x, ref e1, ref e2) if *dt == Dec::DVar && e1.is_value() => {
      let addr = state.alloc(*e1.clone());
      subst(*e2.clone(), *x.clone(), Expr::Addr(addr))
    },
    FnCall(ref v1, ref es) if v1.is_func() => {
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
    Bop(ref op, ref v1, ref e2) if v1.is_value() => {
      Bop(
        op.clone(),
        Box::new(*v1.clone()),
        Box::new(st_step(&mut state, e2))
      )
    },
    Bop(op, e1, e2) => {
      Bop(op, Box::new(st_step(&mut state, &*e1)), e2)
    },
    Assign(ref e1, ref e2, ref e3) if e2.is_value() => {
      Assign(e1.clone(), e2.clone(), Box::new(st_step(&mut state, &*e3)))
    },
    Assign(e1, e2, e3) => {
      Assign(e1, Box::new(st_step(&mut state, &*e2)), e3)
    },
    Uop(op, e1) => {
      Uop(op, Box::new(st_step(&mut state, &*e1)))
    },
    Ternary(e1, e2, e3) => {
      Ternary(Box::new(st_step(&mut state, &*e1)), e2, e3)
    },
    Let(ref e1, ref e2, ref e3) if e1.is_value() => {
      Let(
        Box::new(*e1.clone()),
        Box::new(st_step(&mut state, e2)),
        Box::new(*e3.clone())
      )
    },
    Let(e1, e2, e3) => {
      Let(Box::new(st_step(&mut state, &*e1)), e2, e3)
    },
    Decl(ref dt, ref e1, ref e2, ref e3) if e1.is_value() => {
      Decl(
        dt.clone(),
        Box::new(*e1.clone()),
        Box::new(st_step(&mut state, e2)),
        Box::new(*e3.clone())
      )
    },
    Decl(dt, e1, e2, e3) => {
      Decl(dt, Box::new(st_step(&mut state, &*e1)), e2, e3)
    },
    FnCall(e1, mut xs) => {
      let mut found_first = true;

      for x in xs.iter_mut() {
        if x.is_value() && found_first == true {
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
    if state.expr.is_value() {
      debug!("--- iterations: {}", num_iterations);
      return state.expr
    } else {
      state = step(state);
    }
  }
}
