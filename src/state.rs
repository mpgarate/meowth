use ast::*;
use std::collections::HashMap;

#[derive(Clone, Debug)] 
pub struct Substitution {
  pub x: Expr,
  pub v: Expr,
}

pub struct State {
  addr: usize,
  pub mem: HashMap<usize, Expr>,
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

  pub fn alloc(&mut self, v1: Expr) -> usize {
    let mut addr = self.addr;
    addr += 1;
    self.addr = addr;

    self.mem.insert(addr, v1);

    return self.addr;
  }

  pub fn free(&mut self, addr: usize) {
    self.mem.remove(&addr);
  }

  pub fn assign(&mut self, addr: usize, v1: Expr) {
    self.mem.insert(addr, v1);
  }

  pub fn get(&mut self, addr: usize) -> Expr {
    match self.mem.get(&addr) {
      Some(v) => v.clone(),
      _ => {
        debug!("cannot get addr {:?}", addr);
        panic!("cannot get addr");
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
