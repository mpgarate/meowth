use parser::parser::{parse};
use ast::Expr::*;
use ast::UnOp::*;
use ast::BinOp::*;
use ast::Dec::*;
use ast::*;
use state::*;
use runtime_error::RuntimeError;
use std::result;

pub type Result<T> = result::Result<T, RuntimeError>;

macro_rules! step {
  ($s:expr, $e:expr) => (try!($s.step($e)));
}

macro_rules! to_int {
  ($e:expr) => (try!($e.to_int()));
}

macro_rules! to_var {
  ($e:expr) => (try!($e.to_var()));
}

macro_rules! to_bool {
  ($e:expr) => (try!($e.to_bool()));
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

  pub fn step(&mut self, e: Expr) -> Result<Expr> {
    debug!("step(e) : {:?}", e);
    debug!("step(self.state) : {:?}", self.state.mem);

    let e1 = match e.clone() {
      Var(x) => {
        match self.state.get(x.clone()) {
          Some(e) => e,
          None => return Err(RuntimeError::VariableNotFound(x)),
        }
      },
      /**
       * Values are ineligible for step
       */
      Int(_) | Bool(_) | Func(_, _, _) | Undefined => {
        debug!("stepping on a value {:?}", e);
        return Err(RuntimeError::SteppingOnValue(e));
      },
      /**
       * Base cases
       */
      Uop(Not, ref e1) if e1.is_value() => {
        Bool(!to_bool!(e1))
      },
      Uop(Neg, ref e1) if e1.is_value() => {
        Int(-1 * to_int!(e1))
      },
      Bop(And, ref e1, ref e2) if e1.is_value() && e2.is_value() => {
        Bool(to_bool!(e1) && to_bool!(e2))
      },
      Bop(Or, ref e1, ref e2) if e1.is_value() && e2.is_value() => {
        Bool(to_bool!(e1) || to_bool!(e2))
      },
      Bop(Eq, ref e1, ref e2) if e1.is_value() && e2.is_value() => {
        Bool(*e1 == *e2)
      },
      Bop(Ne, ref e1, ref e2) if e1.is_value() && e2.is_value() => {
        Bool(*e1 != *e2)
      },
      Bop(Mod, ref e1, ref e2) if e1.is_value() && e2.is_value() => {
        let n1 = to_int!(e1);
        let n2 = to_int!(e2);

        // rust % gives the remainder, not modulus
        let result = ((n1 % n2) + n2) % n2;

        Int(result)
      },
      Bop(Lt, ref e1, ref e2) if e1.is_value() && e2.is_value() => {
        Bool(to_int!(e1) < to_int!(e2))
      },
      Bop(Gt, ref e1, ref e2) if e1.is_value() && e2.is_value() => {
        Bool(to_int!(e1) > to_int!(e2))
      },
      Bop(Leq, ref e1, ref e2) if e1.is_value() && e2.is_value() => {
        Bool(to_int!(e1) <= to_int!(e2))
      },
      Bop(Geq, ref e1, ref e2) if e1.is_value() && e2.is_value() => {
        Bool(to_int!(e1) >= to_int!(e2))
      },
      Bop(Plus, ref e1, ref e2) if e1.is_value() && e2.is_value() => {
        Int(to_int!(e1) + to_int!(e2))
      },
      Bop(Minus, ref e1, ref e2) if e1.is_value() && e2.is_value() => {
        Int(to_int!(e1) - to_int!(e2))
      },
      Bop(Times, ref e1, ref e2) if e1.is_value() && e2.is_value() => {
        Int(to_int!(e1) * to_int!(e2))
      },
      Bop(Div, ref e1, ref e2) if e1.is_value() && e2.is_value() => {
        Int(to_int!(e1) / to_int!(e2))
      },
      Bop(Seq, ref v1, ref e2) if v1.is_value() => {
        *e2.clone()
      },
      Bop(Assign, ref v1, ref v2) if v1.is_var() && v2.is_value() => {
        let x = to_var!(v1);
        try!(self.state.assign(x, *v2.clone()));
        debug!("done assigning {:?}", self.state.mem);
        *v2.clone()
      },
      Ternary(ref v1, ref e2, ref e3) if v1.is_value() => {
        match to_bool!(v1) {
          true => *e2.clone(),
          false => *e3.clone(),
        }
      },
      Decl(DConst, ref x, ref v1, ref e2) if v1.is_value() => {
        try!(self.state.alloc_const(to_var!(x), *v1.clone()));
        *e2.clone()
      },
      Decl(DVar, ref x, ref v1, ref e2) if x.is_var() && v1.is_value() => {
        debug!("allocing {:?}", v1);
        try!(self.state.alloc(to_var!(x), *v1.clone()));
        *e2.clone()
      },
      // lambda lift so we can use iter() in guard
      // https://github.com/rust-lang/rfcs/issues/1006
      FnCall(ref v1, ref es) if v1.is_func() && (|| es.iter().all(|v| v.is_value()))() => {
        match **v1 {
          Func(ref name, ref e1, ref xs) => {
            self.state.begin_scope();
            // sub the params
            let exp_result: Result<Expr> = xs.iter().zip(es.iter())
              .fold(Ok(*e1.clone()), |exp, (xn, en)| {
                try!(self.state.alloc(to_var!(xn), en.clone()));
                exp
              });

            let exp = try!(exp_result);

            // sub the fn body for named functions
            let body = match *name {
              None => exp,
              Some(ref s) => {
                try!(self.state.alloc(to_var!(s), *v1.clone()));
                exp
              }
            };
            Scope(Box::new(body))
          },
          _ => return Err(RuntimeError::UnexpectedExpr("expected Func".to_string(), *v1.clone()))
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
          Box::new(step!(self, *e2.clone()))
        )
      },
      Bop(Assign, ref v1, ref e2) if v1.is_var() => {
        Bop(
          Assign,
          Box::new(*v1.clone()),
          Box::new(step!(self, *e2.clone()))
        )
      },
      Bop(op, e1, e2) => {
        Bop(op, Box::new(step!(self, *e1)), e2)
      },
      Uop(op, e1) => {
        Uop(op, Box::new(step!(self, *e1)))
      },
      Ternary(e1, e2, e3) => {
        Ternary(Box::new(step!(self, *e1)), e2, e3)
      },
      While(ref v1, ref e1o, _, ref e2o, ref e3) if v1.is_value() => {
        match to_bool!(v1) {
          true => While(Box::new(*e1o.clone()), e1o.clone(), e2o.clone(), e2o.clone(), e3.clone()),
          false => *e3.clone(),
        }
      },
      While(ref e1, ref e1o, ref v2, ref e2o, ref e3) if v2.is_value() => {
        While(Box::new(step!(self, *e1.clone())), e1o.clone(), v2.clone(), e2o.clone(), e3.clone())
      },
      While(e1, e1o, e2, e2o, e3) => {
        While(e1, e1o, Box::new(step!(self, *e2)), e2o, e3)
      },
      Decl(dt, addr, e1, e2) => {
        Decl(dt, Box::new(*addr.clone()), Box::new(step!(self, *e1)), e2)
      },
      FnCall(ref v1, ref args) if v1.is_value() => {
        let mut found_nonvalue = false;

        let stepped_args: Result<Vec<Expr>> = args.iter().map(|e| {
          if !found_nonvalue && !e.is_value() {
            found_nonvalue = true;
            self.step(e.clone())
          } else {
            Ok(e.clone())
          }
        }).collect();

        match stepped_args {
          Ok(args2) => FnCall(v1.clone(), args2),
          Err(e) => return Err(e)
        }
      },
      FnCall(e1, args) => {
        FnCall(Box::new(step!(self, *e1)), args)
      },
      Scope(e1) => {
        Scope(Box::new(step!(self, *e1)))
      },
    };

    debug!("returning with mem {:?}" , self.state.mem);
    debug!("returning with e {:?}" , e1);
    Ok(e1)
  }

  pub fn eval(&mut self, input: &str) -> Result<Expr> {
    let mut e = try!(parse(input));

    let mut num_iterations = 0;

    loop {
      if num_iterations > 1000 {
        return Err(RuntimeError::TooManyIterations(num_iterations))
      }

      debug!("-----------------");
      debug!("--- iterating on e {:?} ", e);
      debug!("--- iterating on m {:?} ", self.state.mem);
      num_iterations += 1;
      if e.is_value() {
        debug!("--- iterations: {}", num_iterations);
        return Ok(e.clone());
      } else {
        e = try!(self.step(e.clone()));
      }
    }
  }
}

pub fn boxx(input: &str) -> Result<Expr> {
  let mut repl = Repl::new();
  repl.eval(input)
}
