## Building and contributing

This crate requires nightly Rust, as we use unstable features to minimize the
size of the binary as much as possible.

If you're running Linux, it should be as simple as a `cargo run`. If you're
running some other operating system, you'll need to specify a target (or locally
edit the `.cargo/config.toml` file).

### Why is this crate so complicated??

Good question. It's because I decided to turn this into an experiment to get the
smallest binary size available. To do so, we've done some very... drastic
decisions, from not linking against `libc` to compiling the `core` and `alloc`
crates ourselves to optimize for size.
