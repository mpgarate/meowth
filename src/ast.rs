use ast::Expr::*;

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

