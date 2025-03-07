# Timed macro

Use it like this
```rust
    #[timed::timed_instrument(level="debug")]
    fn my_long_method() {

    }
```

and run 
```bash
RUST_LOG=debug cargo run --release
```

and you'll see the log lines with the time in ms 
