# Notes

While developing use this to see debug statements in tests
```sh
RUST_LOG=boxx=debug cargo watch

# for individual tests
RUST_LOG=boxx=debug cargo watch "test test_mut_var"

# with backtrace
RUST_LOG=boxx=debug RUST_BACKTRACE=1 cargo watch "test test_while_loop"

```
### Small notes
* adjust Decl to not require a seq
* create more helpers in parser
* go through and try to clean up clones() and derefs that aren't needed
* go through and think about statement vs block vs binop_expr, adjust usage and naming

### Pokemon
* [easy] print statement
* [breaking] type coercion for bool to number
* [breaking] bike / mutable stack-like var binding
* [breaking] rename a bunch of files and crates

### A few TODOs (post-pokemon)
* [medium] Number type(s) beyond integer
* [easy] language-level exit command
* [medium] exponents **
