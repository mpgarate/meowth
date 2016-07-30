use ast::Expr::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)] 
pub enum UnOp {
  Not,
  Neg,
}

#[derive(Clone, Debug, PartialEq)] 
pub enum BinOp {
  Plus,
  Minus,
  Times,
  Div,
  Eq,
  Ne,
  Leq,
  Geq,
  Lt,
  Gt,
  And,
  Or,
  Mod,
  Seq,
  Assign,
}

#[derive(Clone, Debug, PartialEq)] 
pub enum Dec {
  DVar,
  DConst
}

#[derive(Clone, Debug)] 
pub struct State {
  addr: usize,
  pub mem: Vec<HashMap<usize, Expr>>,
  pub expr: Expr,
}

impl State {
  pub fn from(e: Expr) -> State {
    return State {
      addr: 0,
      mem: vec!(HashMap::new()),
      expr: e,
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

    self.mem[0].insert(addr, v1);

    return self.addr;
  }

  pub fn free(&mut self, addr: usize) {
    match self.mem.iter_mut().find(|m| m.contains_key(&addr)) {
      Some(m) => m.remove(&addr),
      None => {
        debug!("cannot remove; no addr {:?}", addr);
        panic!("cannot remove; no addr")
      },
    };
  }

  pub fn assign(&mut self, addr: usize, v1: Expr) {
    match self.mem.iter_mut().find(|m| m.contains_key(&addr)) {
      Some(m) => m.insert(addr, v1),
      None => {
        debug!("cannot assign; no addr {:?}", addr);
        panic!("cannot assign; no addr")
      }
    };
  }

  pub fn get(&mut self, addr: usize) -> Expr {
    match self.mem.iter().find(|m| m.contains_key(&addr)) {
      Some(m) => m.get(&addr).unwrap().clone(),
      None => {
        debug!("no value for addr {:?}", addr);
        panic!("no value for addr")
      }
    }
  }
}

#[derive(Clone, Debug, PartialEq)] 
pub enum Expr {
  Int(isize),
  Bool(bool),
  Var(String),
  Bop(BinOp, Box<Expr>, Box<Expr>),
  Uop(UnOp, Box<Expr>),
  Ternary(Box<Expr>, Box<Expr>, Box<Expr>),
  Decl(Dec, Box<Expr>, Box<Expr>, Box<Expr>),
  Func(Option<Box<Expr>>, Box<Expr>, Vec<Expr>),
  FnCall(Box<Expr>, Vec<Expr>),
  Addr(usize),
  Scope(Box<Expr>, usize),
}

impl Expr {
  pub fn is_int(&self) -> bool {
    match *self {
      Int(_) => true,
      _ => false,
    }
  }

  pub fn is_bool(&self) -> bool {
    match *self {
      Bool(_) => true,
      _ => false,
    }
  }

  pub fn is_func(&self) -> bool {
    match *self {
      Func(_, _, _) => true,
      _ => false,
    }
  }

  pub fn is_value(&self) -> bool {
    match *self {
      Int(_) | Bool(_) | Var(_) | Func(_, _, _) => true,
      _ => false,
    }
  }

  pub fn is_addr(&self) -> bool {
    match *self {
      Addr(_) => true,
      _ => false,
    }
  }

  pub fn to_int(&self) -> isize {
    match *self {
      Int(n) => n,
      _ => {
        debug!("cant turn into int: {:?}", self);
        panic!()
      }
    }
  }

  pub fn to_bool(&self) -> bool {
    match *self {
      Bool(b) => b,
      _ => {
        debug!("cant turn into bool: {:?}", self);
        panic!()
      }
    }
  }

  pub fn to_addr(&self) -> usize {
    match *self {
      Addr(a) => a,
      _ => {
        debug!("cant turn into addr: {:?}", self);
        panic!()
      }
    }
  }
}

