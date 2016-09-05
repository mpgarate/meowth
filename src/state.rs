use ast::*;
use std::collections::HashMap;

#[derive(Clone, Debug)] 
pub struct State {
  pub mem: HashMap<String, Vec<Expr>>,
}

impl State {
  pub fn new() -> State {
    return State {
      mem: HashMap::new(),
    }
  }

  pub fn alloc(&mut self, x: String, v1: Expr) {
    let vec = &mut self.mem.entry(x).or_insert(Vec::new());

    vec.push(v1);

    debug!("made vec {:?}", vec);
  }

  pub fn free(&mut self, x: String) {
    match self.mem.get_mut(&x) {
      Some(vec) => {
        vec.pop().unwrap();
      },
      None => {
        panic!("cannot free, DNE");
      },
    }
  }

  pub fn assign(&mut self, x: String, v1: Expr) {
    match self.mem.get_mut(&x) {
      Some(vec) => {
        vec.pop();
        vec.push(v1)
      },
      None => panic!(),
    };
  }

  pub fn get(&mut self, x: String) -> Expr {
    match self.mem.get(&x) {
      Some(ref vec) => {
        vec.last().unwrap().clone()
      },
      None => {
        debug!("cannot get variable {:?}", x);
        panic!("cannot get variable")
      },
    }
  }
}
