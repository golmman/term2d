# term2d

## Build, Run Examples

* build everything: `cargo build`
* build library only: `cargo build -p term2d`
* run example `snake`: `cargo run -p snake`

## Contribution

Make sure to format, test and check everything:
```
cargo +nightly fmt && cargo test -p term2d && cargo check
```

Publish with:
```
cargo publish -p term2d
```
