# boxx

While developing use this to see debug statements in tests
```sh
RUST_LOG=boxx=debug cargo watch
```

#### A few TODOs
* variable assignment (let x = 1)
* clean up the parser in general
* split apart lexer and parser?
* refactor expr match thing to group similar binops
* pull tests into separate files
* exit command
* Human-readable errors
* Number type(s) beyond integer
