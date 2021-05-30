#!/bin/sh

curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --verbose
. $HOME/.cargo/env
cargo install statehub
