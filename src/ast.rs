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
}

#[derive(Clone, Debug)] 
pub struct State {
  addr: usize,
  pub mem: HashMap<usize, Expr>,
  pub expr: Expr,
}

impl State {
  pub fn from(e: Expr) -> State {
    return State {
      addr: 0,
      mem: HashMap::new(),
      expr: e,
    }
  }

  pub fn with(&mut self, e1: Expr) -> State {
    let mem = self.mem.clone();
    let addr = self.addr.clone();

    let nextS = State {
      addr: addr,
      mem: mem,
      expr: e1,
    };

    debug!("next state: {:?}", nextS);

    return nextS;
  }

  pub fn alloc(&mut self, v1: Expr) -> usize {
    let mut addr = self.addr;
    addr += 1;
    self.addr = addr;

    self.assign(addr, v1);

    return self.addr;
  }

  pub fn assign(&mut self, addr: usize, v1: Expr) {
    debug!("assigning {:?} as {:?}", addr, v1);
    self.mem.insert(addr, v1);
    debug!("assigned {:?}", self.mem);
  }

  pub fn get(&mut self, addr: usize) -> Expr {
    self.mem.get(&addr).unwrap().clone()
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
  Let(Box<Expr>, Box<Expr>, Box<Expr>),
  VarDecl(Box<Expr>, Box<Expr>, Box<Expr>),
  Func(Option<Box<Expr>>, Box<Expr>, Vec<Expr>),
  FnCall(Box<Expr>, Vec<Expr>),
  Assign(Box<Expr>, Box<Expr>, Box<Expr>),
  Addr(usize),
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

