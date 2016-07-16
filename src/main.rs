extern crate boxx;

use boxx::parser::{parse};
use boxx::expr::{eval};

use std::io::{Write, stdout, stdin};

fn main() {
  loop {
    print!("boxx> ");
    let _ = stdout().flush();

    let mut input = String::new();
    match stdin().read_line(&mut input) {
      Ok(_) => {
        if input == "exit\n".to_string() { 
          break;
        }
        println!("{:?}", eval(parse(&input)))
      },
      Err(e) => print!("error: {}", e)
    }
    let _ = stdout().flush();
  }
}
