use parser::{parse};
use ast::Expr::*;
use ast::UnOp::*;
use ast::BinOp::*;
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
  mem: HashMap<String, String>,
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
    let mem = self.mem.clone();

    return State {
      mem: mem,
      expr: e1,
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
  Let(Box<Expr>, Box<Expr>, Box<Expr>),
  Func(Option<Box<Expr>>, Box<Expr>, Vec<Expr>),
  FnCall(Box<Expr>, Vec<Expr>),
}

pub fn is_int(e: &Expr) -> bool {
  match *e {
    Int(_) => true,
    _ => false,
  }
}

pub fn is_bool(e: &Expr) -> bool {
  match *e {
    Bool(_) => true,
    _ => false,
  }
}

pub fn is_func(e: &Expr) -> bool {
  match *e {
    Func(_, _, _) => true,
    _ => false,
  }
}

pub fn is_value(e: &Expr) -> bool {
  match *e {
    Int(_) | Bool(_) | Var(_) | Func(_, _, _) => true,
    _ => false,
  }
}

pub fn to_int(e: &Expr) -> isize {
  match *e {
    Int(n) => n,
    _ => {
      debug!("cant turn into int: {:?}", e);
      panic!()
    }
  }
}

pub fn to_bool(e: &Expr) -> bool {
  match *e {
    Bool(b) => b,
    _ => {
      debug!("cant turn into bool: {:?}", e);
      panic!()
    }
  }
}

