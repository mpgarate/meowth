use std::fmt;
use std::io;
use std::io::prelude::*;

use std::str::Chars;

#[derive(Debug)]
enum Op {
    Add,
}

#[derive(Debug)] 
enum Value {
    Integer(isize),
}

impl Value {
    fn to_string(&self) -> String {
        match *self {
            Value::Integer(n) => {
                n.to_string()
            }
        }
    }
}

#[derive(Debug)] 
enum Expr {
    Value(Value),
    BinOp(Op, Box<Expr>, Box<Expr>)
}

fn parse_add(line: String) -> Expr {
    let mut chars = line.chars().clone();

    let words: Vec<&str> = line.split('+').collect();

    for i in (0..words.len() / 2) {
        let index = 2*i;

        let n1 = words[index].trim();
        let n2 = words[index + 1].trim();

        let i1: isize = n1
            .parse()
            .ok()
            .expect("Wanted an integer");

        let i2: isize = n2
            .parse()
            .ok()
            .expect("Wanted an integer");

        let e = Expr::BinOp(
            Op::Add,
            Box::new(Expr::Value(Value::Integer(i1))),
            Box::new(Expr::Value(Value::Integer(i2))),
        );

        return e;
    }

    panic!()
}

fn eval(e: Expr) -> Expr {
    match e {
        Expr::BinOp(Op::Add, v1, v2) => {
            match (*v1, *v2) {
                (Expr::Value(Value::Integer(n1)), Expr::Value(Value::Integer(n2))) => {
                    Expr::Value(Value::Integer(n1 + n2))
                }
                _ => panic!()
            }
        }
        _ => panic!()
    }
}

fn eval_expr(line: String) -> String {
    let expr: Expr = parse_add(line);
    let v: Expr = eval(expr);

    match v {
        Expr::Value(v) => v.to_string(),
        _ => panic!(),
    }
}

fn main() {
    loop {
        print!("calc> ");
        io::stdout().flush();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(bytes_read) => {
                println!("{}", eval_expr(input))
            },
            Err(e) => print!("error: {}", e)
        }
        io::stdout().flush();
    }
}
