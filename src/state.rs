use ast::*;
use std::collections::HashMap;

#[derive(Clone, Debug)] 
pub struct Substitution {
  pub x: Expr,
  pub v: Expr,
}

#[derive(Clone, Debug)] 
pub struct State {
  pub mem: HashMap<String, Expr>,
  pub expr: Expr,
}

impl State {
  pub fn from(e: Expr) -> State {
    return State {
      mem: HashMap::new(),
      expr: e,
    }
  }

  pub fn with(&mut self, e1: Expr) -> State {
    return State {
      mem: self.mem.clone(),
      expr: e1,
    }
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

  pub fn merge_mem(&mut self, other: State) {
    for (k,v) in other.mem {
      self.mem.insert(k, v);
    }
  }
}
