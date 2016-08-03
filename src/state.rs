use ast::*;
use std::collections::HashMap;

#[derive(Clone, Debug)] 
pub struct Substitution {
  pub x: Expr,
  pub v: Expr,
}

pub struct State {
  addr: usize,
  pub mem: HashMap<String, Expr>,
  pub expr: Expr,
  pub substitutions: Vec<Substitution>,
}

impl State {
  pub fn from(e: Expr) -> State {
    return State {
      addr: 0,
      mem: HashMap::new(),
      expr: e,
      substitutions: Vec::new(),
    }
  }

  pub fn with(&mut self, e1: Expr) -> &mut State {
    self.expr = e1;
    return self;
  }

  pub fn alloc(&mut self, x: String, v1: Expr) {
    self.mem.insert(x, v1);
  }

  pub fn free(&mut self, x: String) {
    self.mem.remove(&x);
  }

  pub fn assign(&mut self, x: String, v1: Expr) {
    self.mem.insert(x, v1);
  }

  pub fn get(&mut self, x: String) -> Expr {
    match self.mem.get(&x) {
      Some(v) => v.clone(),
      _ => {
        debug!("cannot get x {:?}", x);
        panic!("cannot get x");
      },
    }
  }

  pub fn push_sub(&mut self, x: Expr, v: Expr) {
    debug!("push_sub");
    debug!("x: {:?}", x);
    debug!("v: {:?}", v);
    self.substitutions.push(Substitution { x: x, v: v});
  }
}
