# Ayaya

![AYAYA!](ayaya.jpg)

## Installation

### Building manually

This method is recommended as it produces the smallest binary size, 6732 bytes.

You must be running Linux or Windows. MacOS support not guaranteed. Windows
platform support currently at Tier 3. The instructions below are for Linux,
please modify for your platform as needed.

1. `./build.sh`
2. `mv target/<TARGET_TRIPLE>/release/ayaya ~/.cargo/bin/`

### Via crates.io

This method is discouraged as it produces a larger binary size, 15352 bytes.

You must be running Linux or Windows. MacOS support not guaranteed. Windows
platform support currently at Tier 3.

1. `cargo install --target=<TARGET_TRIPLE> -Z build-std -Z build-std-features=compiler-builtins-mem,panic_immediate_abort ayaya`
2. `ayaya`

`<TARGET_TRIPLE>` represents your current target triple, and must be specified.
For example, on Linux, this would be `x86_64-unknown-linux-gnu`.

## Credits and License

Image generated from [Manytool's Image to ANSI art converter tool][converter].

This work is dual-licensed under Apache 2.0 and GPL 2.0 (or any later version).
You can choose between one of them if you use this work.

`SPDX-License-Identifier: MIT OR Apache-2.0`

[converter]: https://manytools.org/hacker-tools/convert-image-to-ansi-art
