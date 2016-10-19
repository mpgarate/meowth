# boxx

Small-step interpreted programming language, written in rust. 

### Running
Access the REPL
```sh
cargo run
```

Run integration tests
```sh
cargo test
```

### Variable binding
```
var x = 1; // => Undefined
x = 2; // => Undefined
x // => 2
```

### Immutable reference binding
```
let x = 1; // => Undefined
x = 2; // => Error: Cannot assign Int(2) to const x
x
```

### Recursive functions

```
fn fib(n) {
  n == 0 ? 0 : (n == 1 ? 1 : fib(n - 1) + fib(n - 2))
};

fib(8) // => 21
```

Since functions inherit bindings from the outer scope, we can also write:

```
var fib = fn(n) {
   n == 0 ? 0 : (n == 1 ? 1 : fib(n - 1) + fib(n - 2))
};

fib(8) // => 21
```

### Control flow
```
var i = 0;

while (i < 10) {
   if (i % 2 == 0) {
      i = i + 1
   } else {
      i = i + 3
   }
};
i // => 12

```

### No Type Coercion
```
1 + false // => Error: Invalid type conversion. Expected int and found Bool(false)

fn foo(x) { x + 1 }; // => Undefined
foo + 4 // => Error: Invalid type conversion. Expected int and found Func(Some(Var("foo")), Bop(Plus, Var("x"), Int(1)), [Var("x")])
bar(1) + 4 // => Int(6)

```

For more examples and planned features, see the [integration tests](https://github.com/mpgarate/boxx/blob/master/tests/integration.rs). 

### Implementation
Expressions are evaluated using a [small step interpreter](https://github.com/mpgarate/boxx/blob/master/src/expr.rs). With this we can evaluate large expressions piece by piece without going too deep in recursion, while still organizing the code in a functional pattern matching style. 

boxx uses a hand-rolled [recursive descent parser](https://github.com/mpgarate/boxx/blob/master/src/parser/parser.rs) rather than a parser generator in order to have more control of the implementation and avoid dependencies. This is probably the messiest part of the code and source of the trickiest bugs. 

boxx liberally uses Rust's [Box module](https://doc.rust-lang.org/std/boxed/) for heap allocation. This lets us worry a bit less about lifetimes, since the contents are freed when they go out of scope. 

### Features in progress
 - Human-readable errors for parsing and evaluation. Most cases are covered, but the copy could be cleaned up. 
 - Handle floating point numbers
 - Data Types and runtime type checking
