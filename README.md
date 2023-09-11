# term2d

A simple 2d engine for drawing things in your terminal by following a
[nannou](https://github.com/nannou-org/nannou)-like mvc-pattern.

The engine is located in `term2d/`.

For a quick-start pick one of the examples, e.g. `examples/minimal/` or
`examples/dot/`.

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
