#!/bin/sh
cargo test --verbose -- --test-threads 1 "$@"
