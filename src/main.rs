extern crate meowth;

use meowth::interpreter::Interpreter;

use std::io::{Write, stdout, stdin};

fn main() {
  let mut interpreter = Interpreter::new();

  loop {
    print!("meowth :: ");
    let _ = stdout().flush();

    let mut input = String::new();
    match stdin().read_line(&mut input) {
      Ok(_) => {
        if input == String::from("exit\n") {
          break;
        }

        let expr_result = interpreter.eval(&input);
        
        match expr_result {
          Ok(exp) => println!(" => {}", exp), 
          Err(err) => println!("Error: {}", err),
        }
      },
      Err(e) => print!("error: {}", e)
    }
    let _ = stdout().flush();
  }
}
