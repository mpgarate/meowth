use parser::{parse};
use ast::Expr::*;
use ast::UnOp::*;
use ast::BinOp::*;
use ast::Dec::*;
use ast::*;
use state::*;

pub fn subst_all(state: &mut State) {
  for s in state.substitutions.clone() {
    let e = state.expr.clone();
    let esub = substitute(e, s.x, s.v);
    debug!("esub: {:?}", esub);
    state.with(esub);
  }

  debug!("subst_all did this:");
  debug!("subs: {:?}", state.substitutions);
  debug!("expr: {:?}", state.expr);
}

fn subst(s: &mut State, e: Expr, x: Expr, v: Expr) -> Expr {
  s.push_sub(x.clone(), v.clone());
  debug!("inserted subst: {:?}", s.substitutions);
  substitute(e, x, v)
}

fn substitute(e: Expr, x: Expr, v: Expr) -> Expr {
  let sub = |e1: Expr| substitute(e1.clone(), x.clone(), v.clone());

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
    (Scope(e1, a), _) => Scope(Box::new(sub(*e1)), a),
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

pub fn step(state: &mut State) -> &mut State {
  let st_step = |s: &mut State, e1: &Expr| {
    step(s.with(e1.clone())).expr.clone()
  };

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
    Bop(Assign, ref v1, ref v2) if v1.is_addr() && v2.is_value() => {
      let addr = v1.to_addr();
      state.assign(addr.clone(), *v2.clone());
      debug!("done assigning {:?}", state.mem);
      *v2.clone()
    },
    Ternary(ref v1, ref e2, ref e3) if v1.is_value() => {
      match v1.to_bool() {
        true => *e2.clone(),
        false => *e3.clone(),
      }
    },
    Decl(DConst, ref x, ref v1, ref e2) if v1.is_value() => {
      subst(state, *e2.clone(), *x.clone(), *v1.clone())
    },
    Decl(DVar, ref x, ref v1, ref e2) if v1.is_value() => {
      debug!("allocing {:?}", v1);
      let addr = state.alloc(*v1.clone());
      let scope = Scope(Box::new(*e2.clone()), addr);
      subst(state, scope, *x.clone(), Expr::Addr(addr))
    },
    Scope(ref v1, addr) if v1.is_value() => {
      state.free(addr);
      *v1.clone()
    },
    // lambda lift so we can use iter() in guard
    // https://github.com/rust-lang/rfcs/issues/1006
    FnCall(ref v1, ref es) if v1.is_func() && (|| es.iter().all(|v| v.is_value()))() => {
      match **v1 {
        Func(ref name, ref e1, ref xs) => {
          // sub the params
          let exp = xs.iter().zip(es.iter())
            .fold(*e1.clone(), |exp, (xn, en)| subst(state, exp, xn.clone(), en.clone()));

          // sub the fn body for named functions
          match *name {
            None => exp,
            Some(ref s) => subst(state, exp, *s.clone(), *v1.clone())
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
        Box::new(st_step(state, e2))
      )
    },
    Bop(Assign, ref v1, ref e2) if v1.is_addr() => {
      Bop(
        Assign,
        Box::new(*v1.clone()),
        Box::new(st_step(state, e2))
      )
    },
    Bop(op, e1, e2) => {
      Bop(op, Box::new(st_step(state, &*e1)), e2)
    },
    Scope(e1, addr) => {
      Scope(Box::new(st_step(state, &*e1)), addr)
    },
    Uop(op, e1) => {
      Uop(op, Box::new(st_step(state, &*e1)))
    },
    Ternary(e1, e2, e3) => {
      Ternary(Box::new(st_step(state, &*e1)), e2, e3)
    },
    Decl(dt, addr, e1, e2) => {
      Decl(dt, Box::new(*addr.clone()), Box::new(st_step(state, &*e1)), e2)
    },
    FnCall(ref v1, ref args) if v1.is_value() => {
      let mut found_nonvalue = false;

      let args2 = args.iter().map(|e| {
        if !found_nonvalue && !e.is_value() {
          found_nonvalue = true;
          st_step(state, e)
        } else {
          e.clone()
        }
      }).collect();

      FnCall(v1.clone(), args2)
    },
    FnCall(e1, args) => {
      FnCall(Box::new(st_step(state, &*e1)), args)
    },
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
      step(&mut state);
    }
  }
}
