# boxx

While developing use this to see debug statements in tests
```sh
RUST_LOG=boxx=debug cargo watch
```

#### A few TODOs
* mod, and, or
* exit command
* make the parser better with multi-character token matching
* make the parser smarter about binop matching
* clean up the parser in general
* split apart lexer and parser?
* pull tests into separate files
* ternary op (if statement)
* expression chain (e1; e2)
* variable assignment (let x = 1)
* Number type(s) beyond integer
* Human-readable errors
