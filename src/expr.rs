use parser::{parse};
use ast::Expr::*;
use ast::UnOp::*;
use ast::BinOp::*;
use ast::Dec::*;
use ast::*;
use state::*;

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
    (Undefined, _) => e,
    (Bop(op, e1, e2), _) => { 
      Bop(
        op,
        Box::new(sub(*e1)),
        Box::new(sub(*e2))
      )
    },
    (Uop(op, e1), _) => Uop(op, Box::new(sub(*e1))),
    (Scope(e1), _) => Scope(Box::new(sub(*e1))),
    (Ternary(e1, e2, e3), _) => {
      Ternary(
        Box::new(sub(*e1)),
        Box::new(sub(*e2)),
        Box::new(sub(*e3))
      )
    },
    (While(e1, e1o, e2, e2o, e3), _) => {
      While(
        Box::new(sub(*e1)),
        Box::new(sub(*e1o)),
        Box::new(sub(*e2)),
        Box::new(sub(*e2o)),
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

pub struct Repl {
  pub state: State,
}

impl Repl {
  pub fn new() -> Repl {
    Repl {
      state: State::new(),
    }
  }

  pub fn step(&mut self, e: Expr) -> Expr {
    debug!("step(e) : {:?}", e);
    debug!("step(self.state) : {:?}", self.state.mem);

    let e1 = match e.clone() {
      Var(x) => {
        self.state.get(x)
      },
      /**
       * Values are ineligible for step
       */
      Int(_) | Bool(_) | Func(_, _, _) | Undefined => {
        debug!("stepping on a value {:?}", e);
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
      Bop(Assign, ref v1, ref v2) if v1.is_var() && v2.is_value() => {
        let x = v1.to_var();
        self.state.assign(x, *v2.clone());
        debug!("done assigning {:?}", self.state.mem);
        *v2.clone()
      },
      Ternary(ref v1, ref e2, ref e3) if v1.is_value() => {
        match v1.to_bool() {
          true => *e2.clone(),
          false => *e3.clone(),
        }
      },
      Decl(DConst, ref x, ref v1, ref e2) if v1.is_value() => {
        self.state.alloc_const(x.to_var(), *v1.clone());
        *e2.clone()
      },
      Decl(DVar, ref x, ref v1, ref e2) if x.is_var() && v1.is_value() => {
        debug!("allocing {:?}", v1);
        self.state.alloc(x.to_var(), *v1.clone());
        *e2.clone()
      },
      // lambda lift so we can use iter() in guard
      // https://github.com/rust-lang/rfcs/issues/1006
      FnCall(ref v1, ref es) if v1.is_func() && (|| es.iter().all(|v| v.is_value()))() => {
        match **v1 {
          Func(ref name, ref e1, ref xs) => {
            // sub the params
            let exp = xs.iter().zip(es.iter())
              .fold(*e1.clone(), |exp, (xn, en)| subst(exp, xn.clone(), en.clone()));

            // sub the fn body for named functions
            let body = match *name {
              None => exp,
              Some(ref s) => subst(exp, *s.clone(), *v1.clone()),
            };

            self.state.begin_scope();
            Scope(Box::new(body))
          },
          _ => {
            debug!("expected a Func, got {:?}", v1);
            panic!()
          },
        }
      },
      Scope(ref v1) if v1.is_value() => {
        self.state.end_scope();
        *v1.clone()
      },
      /**
       * Search Cases
       */
      Bop(ref op, ref v1, ref e2) if v1.is_value() => {
        Bop(
          op.clone(),
          Box::new(*v1.clone()),
          Box::new(self.step(*e2.clone()))
        )
      },
      Bop(Assign, ref v1, ref e2) if v1.is_var() => {
        Bop(
          Assign,
          Box::new(*v1.clone()),
          Box::new(self.step(*e2.clone()))
        )
      },
      Bop(op, e1, e2) => {
        Bop(op, Box::new(self.step(*e1)), e2)
      },
      Uop(op, e1) => {
        Uop(op, Box::new(self.step(*e1)))
      },
      Ternary(e1, e2, e3) => {
        Ternary(Box::new(self.step(*e1)), e2, e3)
      },
      While(ref v1, ref e1o, _, ref e2o, ref e3) if v1.is_value() => {
        match v1.to_bool() {
          true => While(Box::new(*e1o.clone()), e1o.clone(), e2o.clone(), e2o.clone(), e3.clone()),
          false => *e3.clone(),
        }
      },
      While(ref e1, ref e1o, ref v2, ref e2o, ref e3) if v2.is_value() => {
        While(Box::new(self.step(*e1.clone())), e1o.clone(), v2.clone(), e2o.clone(), e3.clone())
      },
      While(e1, e1o, e2, e2o, e3) => {
        While(e1, e1o, Box::new(self.step(*e2)), e2o, e3)
      },
      Decl(dt, addr, e1, e2) => {
        Decl(dt, Box::new(*addr.clone()), Box::new(self.step(*e1)), e2)
      },
      FnCall(ref v1, ref args) if v1.is_value() => {
        let mut found_nonvalue = false;

        let args2 = args.iter().map(|e| {
          if !found_nonvalue && !e.is_value() {
            found_nonvalue = true;
            self.step(e.clone())
          } else {
            e.clone()
          }
        }).collect();

        FnCall(v1.clone(), args2)
      },
      FnCall(e1, args) => {
        FnCall(Box::new(self.step(*e1)), args)
      },
      Scope(e1) => {
        Scope(Box::new(self.step(*e1)))
      },
    };

    debug!("returning with mem {:?}" , self.state.mem);
    debug!("returning with e {:?}" , e1);
    e1
  }

  pub fn eval(&mut self, input: &str) -> Expr {
    let mut e = parse(input);

    let mut num_iterations = 0;

    loop {
      if num_iterations > 1000 {
        panic!("too many step iterations");
      }

      debug!("-----------------");
      debug!("--- iterating on e {:?} ", e);
      debug!("--- iterating on m {:?} ", self.state.mem);
      num_iterations += 1;
      if e.is_value() {
        debug!("--- iterations: {}", num_iterations);
        return e.clone();
      } else {
        e = self.step(e);
      }
    }
  }
}

pub fn boxx(input: &str) -> Expr {
  let mut repl = Repl::new();
  repl.eval(input)
}
