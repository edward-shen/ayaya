#!/usr/bin/env bash

cargo build --release --features smaller

upx --best target/x86_64-unknown-linux-gnu/release/ayaya