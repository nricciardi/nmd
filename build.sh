#!/bin/sh

rustup update

cargo update


cargo build --release --target x86_64-unknown-linux-gnu

cargo build --release --target x86_64-pc-windows-gnu
