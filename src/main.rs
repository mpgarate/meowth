mod parser;
mod expr;


/*
 *

fn integer<'a, I>(input: State<I>) -> ParseResult<Expr, I>
  where I: Stream<Item = char>
{
  let (s, input) = try!(many1::<String, _>(digit())
                        .expected("integer")
                        .parse_state(input));
  let mut n = 0;
  for c in s.chars() {
    n = n * 10 + (c as i64 - '0' as i64);
  }
  Ok((Expr::Value(Value::Integer(n)), input))
}

fn expr<I>(input: State<I>) -> ParseResult<Expr, I> 
  where I: Stream<Item=char>
{
  /*
  let integer = many1(digit())
    .map(|s: String|
      Expr::Value(Value::Integer(s.parse::<i64>().unwrap()))
    );
    */

  let lex_char = |c| char(c).skip(spaces());

  let integer_add = (
      parser(integer),
      lex_char('+'),
      parser(integer),
    ).map(|t| Expr::BinOp(Op::Add, Box::new(t.0), Box::new(t.2)));

    integer_add
    .or(parser(integer))
    .skip(spaces())
    .parse_state(input)
}

fn eval_expr(line: String) -> String {
  let result = parser(expr).parse(&*line);

  match result {
    Ok((e, remaining)) => match eval(e) {
      Expr::Value(v) => v.to_string(),
      _ => panic!(),
    },
    Err(e) => {
      println!("{:?}", e);
      panic!()
    }
  }
}

fn main() {
  loop {
    print!("calc> ");
    io::stdout().flush();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
      Ok(bytes_read) => {
        println!("{}", parser::parse(input))
      },
      Err(e) => print!("error: {}", e)
    }
    io::stdout().flush();
  }
}
*/
