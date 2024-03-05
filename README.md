## axum-tutorial
- tutorial by Jeremy Chone
  - basic: https://www.youtube.com/watch?v=XZtlD_m59sM
  - production: https://www.youtube.com/watch?v=3cA_mk4vdWY 

### How to run
Install [cargo-watch](https://crates.io/crates/cargo-watch)
```sh
cargo install cargo-watch
```

Prepare two terminal.

For client,
```sh
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```

For server,
```sh
cargo watch -q -c -w src/ -x run
```
