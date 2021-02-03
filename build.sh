#!/usr/bin/env bash

set -euxo pipefail

TARGET="x86_64-unknown-linux-gnu"
OUT_FILE="target/$TARGET/release/ayaya"

cargo build --release

objcopy -R .note.gnu.build-id \
	-R .dynsym \
	-R .dynstr \
	-R .eh_frame_hdr \
	-R .eh_frame \
	-R .got \
	"$OUT_FILE"

strip "$OUT_FILE"

upx --ultra-brute "$OUT_FILE"
