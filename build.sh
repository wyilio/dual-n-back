#!/bin/sh
cargo build -v --target x86_64-pc-windows-gnu &&
cp target/x86_64-pc-windows-gnu/debug/dual-n-back.exe . &&
exec ./dual-n-back.exe "$@"
