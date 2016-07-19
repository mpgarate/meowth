# boxx

While developing use this to see debug statements in tests
```sh
RUST_LOG=boxx=debug cargo watch

```
### Small notes
* be consistent about to_box vs Box::new in parser
* create more helpers in parser?
* go through and try to clean up clones() and derefs that aren't needed

### Pokemon
* [easy] allow underscores in var / fn names
* [medium] function parameters
* [medium] if, else if, else (separate from ternary)
* [easy] print statement
* [hard] while loop
* [hard] at runtime, var name should be accessible (not purely substituted)
* [hard] REPL should maintain state
* [medium] ability to run standalone program files through interpreter
* [breaking] type coercion for bool to number
* [breaking] bike / mutable stack-like var binding
* [breaking] rename a bunch of files and crates

### A few TODOs (post-pokemon)
* [medium] Number type(s) beyond integer
* [hard] Human-readable errors
* [easy] language-level exit command
* [medium] exponents **
