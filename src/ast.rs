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
  While(Box<Expr>, Box<Expr>, Box<Expr>, Box<Expr>, Box<Expr>),
  Decl(Dec, Box<Expr>, Box<Expr>, Box<Expr>),
  Func(Option<Box<Expr>>, Box<Expr>, Vec<Expr>),
  FnCall(Box<Expr>, Vec<Expr>),
  Scope(Box<Expr>, String),
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

  pub fn is_var(&self) -> bool {
    match *self {
      Var(_) => true,
      _ => false,
    }
  }

  pub fn is_value(&self) -> bool {
    match *self {
      Int(_) | Bool(_) | Func(_, _, _) => true,
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

  pub fn to_var(&self) -> String {
    match *self {
      Var(ref x) => x.clone(),
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
}

