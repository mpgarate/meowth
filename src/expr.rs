use expr::Expr::*;
use runtime_error::RuntimeError;
use std::fmt;

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
  Undefined,
  Bop(BinOp, Box<Expr>, Box<Expr>),
  Uop(UnOp, Box<Expr>),
  Ternary(Box<Expr>, Box<Expr>, Box<Expr>),
  While(Box<Expr>, Box<Expr>, Box<Expr>, Box<Expr>, Box<Expr>),
  Decl(Dec, Box<Expr>, Box<Expr>, Box<Expr>),
  Func(Option<Box<Expr>>, Box<Expr>, Vec<Expr>),
  FnCall(Box<Expr>, Vec<Expr>),
  Scope(Box<Expr>),
  Print(Box<Expr>),
  PrintVarName(Box<Expr>),
}

impl Expr {
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
      Int(_) | Bool(_) | Func(_, _, _) | Undefined => true,
      _ => false,
    }
  }

  pub fn to_int(&self) -> Result<isize, RuntimeError> {
    match *self {
      Int(n) => Ok(n),
      _ => Err(RuntimeError::InvalidTypeConversion("int".to_string(), self.clone())),
    }
  }

  pub fn to_var(&self) -> Result<String, RuntimeError> {
    match *self {
      Var(ref x) => Ok(x.clone()),
      _ => Err(RuntimeError::InvalidTypeConversion("var".to_string(), self.clone())),
    }
  }

  pub fn to_bool(&self) -> Result<bool, RuntimeError> {
    match *self {
      Bool(b) => Ok(b),
      _ => Err(RuntimeError::InvalidTypeConversion("bool".to_string(), self.clone())),
    }
  }
}

impl fmt::Display for Expr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Int(n) => write!(f, "{}", n),
      Bool(true) => write!(f, "win"),
      Bool(false) => write!(f, "lose"),
      _ => write!(f, "cannot print this thing")
    }
  }
}
