extern crate boxx;

use boxx::expr::Repl;

use std::io::{Write, stdout, stdin};

fn main() {
  let mut repl = Repl::new();

  loop {
    print!("boxx> ");
    let _ = stdout().flush();

    let mut input = String::new();
    match stdin().read_line(&mut input) {
      Ok(_) => {
        if input == "exit\n".to_string() { 
          break;
        }
        println!("{:?}", repl.eval(&input))
      },
      Err(e) => print!("error: {}", e)
    }
    let _ = stdout().flush();
  }
}
