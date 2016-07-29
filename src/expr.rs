use parser::{parse};
use ast::Expr::*;
use ast::UnOp::*;
use ast::BinOp::*;
use ast::Dec::*;
use ast::*;

fn subst(e: Expr, x: Expr, v: Expr) -> Expr {
  let sub = |e1: Expr| subst(e1.clone(), x.clone(), v.clone());

  match (e.clone(), x.clone()) {
    (Var(ref s1), Var(ref s2)) if s1 == s2 => v.clone(),
    (FnCall(v1, xs), _) => {
      let xs2 = xs.iter().map(|xn| sub(xn.clone())).collect();

      FnCall(Box::new(sub(*v1)), xs2)
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
    (Decl(d, y, e1, e2), _) => {
      let e2s = if *y == x {
        *e2
      } else {
        sub(*e2)
      };

      Decl(
        d,
        Box::new(*y),
        Box::new(sub(*e1)),
        Box::new(e2s)
      )
    },
    (Func(name, e1, xs), _) => {
      match xs.iter().find(|y| **y == x) {
        Some(_) => e,
        None if name == Some(Box::new(x.clone())) => e,
        None => Func(name, Box::new(sub(*e1)), xs)
      }
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
    Bop(Seq, ref v1, ref e2) if v1.is_value() => {
      *e2.clone()
    },
    Assign(ref v1, ref v2, ref e3) if v1.is_addr() && v2.is_value() => {
      let addr = v1.to_addr();
      state.assign(addr.clone(), *v2.clone());
      debug!("done assigning {:?}", state.mem);
      *e3.clone()
    },
    Ternary(ref v1, ref e2, ref e3) if v1.is_value() => {
      match v1.to_bool() {
        true => *e2.clone(),
        false => *e3.clone(),
      }
    },
    Decl(ref dconst, ref x, ref v1, ref e2) if *dconst == DConst && v1.is_value() => {
      subst(*e2.clone(), *x.clone(), *v1.clone())
    },
    Decl(ref dvar, ref x, ref v1, ref e2) if *dvar == DVar && v1.is_value() => {
      let addr = state.alloc(*v1.clone());
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
    /*
     * TODO: should there be a case like this?
    Assign(ref e1, ref v2, ref e3) if v2.is_value() => {
      Assign(e1.clone(), v2.clone(), Box::new(st_step(&mut state, &*e3)))
    },
    */
    Assign(e1, e2, e3) => {
      Assign(e1, Box::new(st_step(&mut state, &*e2)), e3)
    },
    Uop(op, e1) => {
      Uop(op, Box::new(st_step(&mut state, &*e1)))
    },
    Ternary(e1, e2, e3) => {
      Ternary(Box::new(st_step(&mut state, &*e1)), e2, e3)
    },
    /*
     * TODO: should there be a case like this?
    Decl(ref dt, ref addr, ref v1, ref e2) if v1.is_value() => {
      Decl(
        dt.clone(),
        Box::new(*addr.clone()),
        Box::new(*v1.clone()),
        Box::new(st_step(&mut state, e2))
      )
    },
    */
    Decl(dt, addr, e1, e2) => {
      Decl(dt, Box::new(*addr.clone()), Box::new(st_step(&mut state, &*e1)), e2)
    },
    /*
     * TODO: should there be a case like this?
    FnCall(ref v1, ref mut args) if v1.is_value() => {
      let mut found_first = true;

      for x in args.iter_mut() {
        if x.is_value() && found_first == true {
          found_first = false;
          *x = st_step(&mut state, x);
        }
      }

      FnCall(Box::new(*v1.clone()), args.clone())
    }
    */
    FnCall(e1, args) => {
      FnCall(Box::new(st_step(&mut state, &*e1)), args)
    }
  };

  debug!("returning with mem {:?}" , state.mem);
  debug!("returning with e {:?}" , state.expr);
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
      state = step(state)
    }
  }
}
