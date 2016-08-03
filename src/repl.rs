use ast::*;
use expr::{step};
use parser::{parse};
use state::{State};

pub struct Repl {
  state: State
}

impl Repl {
  pub fn new() -> Repl {
    return Repl {
      // TODO: it feels hacky to start off the state with a zero
      state: State::from(Expr::Int(0))
    };
  }

  pub fn eval(&mut self, input: &str) -> Expr {
    self.state.with(parse(input));
    //subst_all(&mut self.state);

    loop {
      if self.state.expr.is_value() {
        return self.state.expr.clone()
      } else {
        step(&mut self.state);
      }
    }
  }
}
