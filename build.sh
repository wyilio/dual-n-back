#!/bin/sh
cargo build --features bevy/dynamic_linking --target x86_64-pc-windows-gnu &&
cp target/x86_64-pc-windows-gnu/debug/dual-n-back.exe . &&
exec ./dual-n-back.exe "$@"
