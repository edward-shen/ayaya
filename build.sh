#!/usr/bin/env bash

cargo build --release --features smaller

FILE_NAME="target/x86_64-unknown-linux-gnu/release/ayaya"

objcopy -R .note.gnu.build-id \
	-R .dynsym \
	-R .dynstr \
	-R .eh_frame_hdr \
	-R .eh_frame \
	-R .got \
	"$FILE_NAME"

upx --best "$FILE_NAME"
