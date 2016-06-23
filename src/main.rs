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
    Integer(u32),
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

impl Expr {
    fn to_value_string(&self) -> String {
        match *self {
            Expr::Value(ref v) => v.to_string(),
            _ => panic!(),
        }
    }
}

fn parse_add(line: String) -> Expr {
    let mut chars = line.chars().clone();

    let operand1 = chars.next();
    let operator = chars.next();
    let operand2 = chars.next();

    match (operand1, operand2) {
        (Some(op1), Some(op2)) => {
            Expr::BinOp(
                Op::Add,
                Box::new(Expr::Value(Value::Integer(op1.to_digit(10).unwrap()))),
                Box::new(Expr::Value(Value::Integer(op2.to_digit(10).unwrap()))),
            )
        }
        _ => Expr::Value(Value::Integer(-1))
    }
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
    let expr = parse_add(line);
    let v: Expr = eval(expr);

    return v.to_value_string();
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
