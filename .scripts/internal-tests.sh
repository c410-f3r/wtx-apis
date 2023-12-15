#!/usr/bin/env bash

set -euxo pipefail

cargo install rust-tools --git https://github.com/c410-f3r/regular-crates

rt='rust-tools --template you-rust'

export CARGO_TARGET_DIR="$($rt target-dir)"
export RUST_BACKTRACE=1
export RUSTFLAGS="$($rt rust-flags -Asingle_use_lifetimes "")"

$rt rustfmt
$rt clippy

$rt check-generic .
$rt check-with-features . aptos
$rt check-with-features . async-send
$rt check-with-features . base64
$rt check-with-features . bincode
$rt check-with-features . bs58
$rt check-with-features . default
$rt check-with-features . ed25519-dalek
$rt check-with-features . ethabi
$rt check-with-features . ethereum
$rt check-with-features . ethereum-types
$rt check-with-features . hex
$rt check-with-features . json-placeholder
$rt check-with-features . nager-date
$rt check-with-features . primitive-types
$rt check-with-features . rick-and-morty
$rt check-with-features . solana
$rt check-with-features . solana-program
$rt check-with-features . std
