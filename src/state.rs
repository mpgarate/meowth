use expr::Expr;
use std::collections::HashMap;
use runtime_error::RuntimeError;

#[derive(Clone, Debug)] 
pub struct State {
  pub mem: Vec<HashMap<String, Binding>>,
}

#[derive(Clone, Debug)] 
pub enum Binding {
  Bike(Vec<Box<Expr>>),
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
    let binding = Binding::Bike(vec!(Box::new(v1)));

    match self.mem.last_mut() {
      Some(map) => {
        map.insert(x, binding);
        Ok(())
      },
      None => Err(RuntimeError::InvalidMemoryState("no memory frame for var allocation".to_string())),
    }
  }

  pub fn alloc_const(&mut self, x: String, v1: Expr) ->Result<(), RuntimeError> {
    let binding = Binding::Const(Box::new(v1));

    match self.mem.last_mut() {
      Some(map) => {
        map.insert(x, binding);
        Ok(())
      },
      None => return Err(RuntimeError::InvalidMemoryState("no memory frame for const allocation".to_string())),
    }
  }

  pub fn assign(&mut self, x: String, v1: Expr) -> Result<(), RuntimeError> {
    let mut map_option = self.first_map_for(x.clone());

    let map = match map_option.as_mut() {
      Some(m) => m,
      None => return Err(RuntimeError::InvalidConstAssignment(v1, x)),
    };

    match map.get_mut(&x) {
      Some(&mut Binding::Bike(ref mut v)) => {
        v.push(Box::new(v1));
      },
      _ => return Err(RuntimeError::InvalidConstAssignment(v1, x))
    };

    Ok(())
  }

  pub fn get(&mut self, x: String) -> Result<Expr, RuntimeError> {
    match self.first_map_for(x.clone()) {
      Some(map) => match map.get(&x).clone() {
        Some(&Binding::Bike(ref b)) => {
          match b.last() {
            Some(e) => Ok(*e.clone()),
            _ => Err(RuntimeError::EmptyBike(x)),
          }
        }
        Some(&Binding::Const(ref e)) => Ok(*e.clone()),
        _ => Err(RuntimeError::VariableNotFound(x)),
      },
      None => Err(RuntimeError::VariableNotFound(x)),
    }
  }

  pub fn contains(&mut self, x: String) -> bool {
    match self.get(x) {
      Ok(_) => true,
      _ => false,
    }
  }

  pub fn give(&mut self, x: String) -> Result<Expr, RuntimeError> {
    match self.first_map_for(x.clone()) {
      Some(map) => match map.get_mut(&x) {
        Some(binding) => match binding {
          &mut Binding::Bike(ref mut v) => {
            match v.pop() {
              Some(e) => Ok(*e),
              None => Err(RuntimeError::EmptyBike(x)),
            }
          }
          _ => Err(RuntimeError::GiveFromConst(x)),
        },
        _ => Err(RuntimeError::VariableNotFound(x)),
      },
      None => Err(RuntimeError::VariableNotFound(x)),
    }
  }

  pub fn begin_scope(&mut self) {
    self.mem.push(HashMap::new());
  }

  pub fn end_scope(&mut self) {
    self.mem.pop();
  }
}
