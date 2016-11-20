# Notes

While developing use this to see debug statements in tests
```sh
RUST_LOG=meowth=debug cargo watch

# for individual tests
RUST_LOG=meowth=debug cargo watch "test test_mut_var"

# with backtrace
RUST_LOG=meowth=debug RUST_BACKTRACE=1 cargo watch "test test_while_loop"
```
