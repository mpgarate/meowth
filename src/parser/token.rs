#[derive(Clone, Debug, PartialEq)] 
pub enum Token {
  Plus,
  Minus,
  Times,
  Div,
  RParen,
  LParen,
  Eq,
  Ne,
  Leq,
  Geq,
  Lt,
  Gt,
  Not,
  And,
  Or,
  Mod,
  Seq,
  Ternary,
  Else,
  Var(String),
  Int(isize),
  Bool(bool),
  Let,
  VarDecl,
  Assign,
  FnDecl,
  LBracket,
  RBracket,
  Comma,
  If,
  While,
  EOF,
  Print,
  PrintVarName,
  Give,
}

impl Token {
  pub fn is_term_op(&self) -> bool {
    match *self {
      Token::Times => true,
      Token::Div => true,
      _ => false,
    }
  
  }

  pub fn is_expr_op(&self) -> bool {
    match *self {
      Token::Plus => true,
      Token::Minus => true,
      Token::Eq => true,
      Token::Ne => true,
      Token::Leq => true,
      Token::Geq => true,
      Token::Lt => true,
      Token::Gt => true,
      Token::And => true,
      Token::Or => true,
      Token::Mod => true,
      _ => false,
    }
  }

  pub fn is_statement_op(&self) -> bool {
    match *self {
      Token::Ternary => true,
      Token::Assign => true,
      _ => false,
    }
  }

  pub fn is_block_op(&self) -> bool {
    match *self {
      Token::Seq => true,
      _ => false,
    }
  }
}

