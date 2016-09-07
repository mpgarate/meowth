use ast::*;
use std::collections::HashMap;

#[derive(Clone, Debug)] 
pub struct State {
  pub mem: Vec<HashMap<String, Binding>>,
}

#[derive(Clone, Debug)] 
pub enum Binding {
  Var(Box<Expr>),
  Const(Box<Expr>),
}

impl State {
  pub fn new() -> State {
    return State {
      mem: vec!(HashMap::new()),
    }
  }

  fn first_map_for(&mut self, x: String) -> Option<&mut HashMap<String, Binding>> {
    self.mem.iter_mut().rev().find(|map| map.contains_key(&x))
  }

  pub fn alloc(&mut self, x: String, v1: Expr) {
    let binding = Binding::Var(Box::new(v1));
    self.mem.last_mut().unwrap().insert(x, binding);
  }

  pub fn alloc_const(&mut self, x: String, v1: Expr) {
    let binding = Binding::Const(Box::new(v1));

    self.mem.last_mut().unwrap().insert(x, binding);
  }

  pub fn assign(&mut self, x: String, v1: Expr) {
    let mut map_option = self.first_map_for(x.clone());
    let map = map_option.as_mut().unwrap();

    let binding = map.get_mut(&x).unwrap().clone();

    match binding {
      Binding::Var(_) => map.insert(x, Binding::Var(Box::new(v1))),
      Binding::Const(_) => panic!("cannot assign to const"),
    };
  }

  pub fn get(&mut self, x: String) -> Expr {
    let binding = self.first_map_for(x.clone()).unwrap().get(&x).unwrap().clone();

    match binding.clone() {
      Binding::Var(e) => *e,
      Binding::Const(e) => *e,
    }
  }

  pub fn begin_scope(&mut self) {
    self.mem.push(HashMap::new());
  }

  pub fn end_scope(&mut self) {
    self.mem.pop();
  }
}
