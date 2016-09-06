use ast::*;
use std::collections::HashMap;

#[derive(Clone, Debug)] 
pub struct State {
  pub mem: Vec<HashMap<String, Expr>>,
}

impl State {
  pub fn new() -> State {
    return State {
      mem: vec!(HashMap::new()),
    }
  }

  fn first_map_for(&mut self, x: String) -> Option<&mut HashMap<String, Expr>> {
    self.mem.iter_mut().rev().find(|map| map.contains_key(&x))
  }

  pub fn alloc(&mut self, x: String, v1: Expr) {
    self.mem.last_mut().unwrap().insert(x, v1);
  }

  pub fn assign(&mut self, x: String, v1: Expr) {
    self.first_map_for(x.clone()).as_mut().unwrap().insert(x, v1);
  }

  pub fn get(&mut self, x: String) -> Expr {
    self.first_map_for(x.clone()).unwrap().get(&x).unwrap().clone()
  }

  pub fn begin_scope(&mut self) {
    self.mem.push(HashMap::new());
  }

  pub fn end_scope(&mut self) {
    self.mem.pop();
  }
}
