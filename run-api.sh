#!/bin/bash
export PATH="$HOME/.cargo/bin:$PATH"
cargo run api --no-auth
exec bash
