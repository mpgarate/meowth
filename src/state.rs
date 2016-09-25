use ast::*;
use std::collections::HashMap;
use runtime_error::RuntimeError;

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

  pub fn alloc(&mut self, x: String, v1: Expr) -> Result<(), RuntimeError> {
    let binding = Binding::Var(Box::new(v1));

    match self.mem.last_mut() {
      Some(map) => { map.insert(x, binding); () },
      None => return Err(RuntimeError::InvalidMemoryState("no memory frame for var allocation".to_string())),
    }

    Ok(())
  }

  pub fn alloc_const(&mut self, x: String, v1: Expr) ->Result<(), RuntimeError> {
    let binding = Binding::Const(Box::new(v1));

    match self.mem.last_mut() {
      Some(map) => { map.insert(x, binding); () },
      None => return Err(RuntimeError::InvalidMemoryState("no memory frame for const allocation".to_string())),
    }

    Ok(())
  }

  pub fn assign(&mut self, x: String, v1: Expr) -> Result<(), RuntimeError> {
    let mut map_option = self.first_map_for(x.clone());
    let map = map_option.as_mut().unwrap();

    let binding = map.get_mut(&x).unwrap().clone();

    match binding {
      Binding::Var(_) => map.insert(x, Binding::Var(Box::new(v1))),
      Binding::Const(_) => return Err(RuntimeError::InvalidConstAssignment(v1, x)),
    };
    Ok(())
  }

  pub fn get(&mut self, x: String) -> Option<Expr> {
    match self.first_map_for(x.clone()) {
      Some(map) => match map.get(&x).unwrap().clone() {
        Binding::Var(e) => Some(*e),
        Binding::Const(e) => Some(*e),
      },
      None => None,
    }
  }

  pub fn begin_scope(&mut self) {
    self.mem.push(HashMap::new());
  }

  pub fn end_scope(&mut self) {
    self.mem.pop();
  }
}
