# Ayaya

![AYAYA!](ayaya.jpg)

## Installation

You must be running Linux. MacOS support not guaranteed.

1. `cargo install ayaya`
2. `ayaya`

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

## Credits and License

Image generated from [Manytool's Image to ANSI art converter tool][converter].

This work is dual-licensed under Apache 2.0 and GPL 2.0 (or any later version).
You can choose between one of them if you use this work.

`SPDX-License-Identifier: MIT OR Apache-2.0`

[converter]: https://manytools.org/hacker-tools/convert-image-to-ansi-art
