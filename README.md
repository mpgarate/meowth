# boxx

Small-step interpreted programming language, written in rust. 

### Running
Access the REPL with `cargo run`
Run integration tests `cargo test`

### Variable binding
```
var x = 1;
x = 2;
x // => 2
```

### Immutable reference binding
```
let x = 1;
x = 2; // => err, invalid assignment
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

### Features in progress
 - Human-readable errors for parsing and evaluation
 - Handle floating point numbers
