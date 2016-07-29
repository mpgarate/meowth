# boxx

While developing use this to see debug statements in tests
```sh
RUST_LOG=boxx=debug cargo watch

# for individual tests
RUST_LOG=boxx=debug cargo watch "test test_mut_var"

```
### Small notes
* create more helpers in parser
* go through and try to clean up clones() and derefs that aren't needed
* go through and think about statement vs block vs binop_expr, adjust usage and naming

### Pokemon
* [hard] at runtime, var name should be accessible (not purely substituted)
* [hard] REPL should maintain state
  * consider using Mem https://github.com/mpgarate/ProgLang-Assignments/blob/master/HW6/src/main/scala/js/hw6/ast.scala#L135
* [easy] print statement
* [hard] while loop
* [medium] ability to run standalone program files through interpreter
* [breaking] type coercion for bool to number
* [breaking] bike / mutable stack-like var binding
* [breaking] rename a bunch of files and crates

### A few TODOs (post-pokemon)
* [medium] Number type(s) beyond integer
* [hard] Human-readable errors
* [easy] language-level exit command
* [medium] exponents **
