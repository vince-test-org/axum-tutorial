## axum-tutorial
- tutorial by Jeremy Chone - https://www.youtube.com/watch?v=XZtlD_m59sM

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
