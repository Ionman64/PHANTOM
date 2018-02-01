#!/usr/bin/env bash
echo "*** Testing all modules ***\n\n"
cargo test

echo "*** Done ***\n"

echo "*** Running ignored test methods in module database on 1 thread ***\n\n"
cargo test database -- --ignored --test-threads=1

echo "*** Done ***"
