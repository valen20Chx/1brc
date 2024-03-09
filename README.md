# 1brc - One Billion Row Challenge

This is a rust solution for the One Billion Row Challenge.

## Init

Run `bash init.sh` to clone the original repository of the challenge and generate the data, then move it to our directory.

## Run

Run `cargo run` to solve the challenge.

## Benchmarking

You can benchmark the solution with cargo-flamegraph.
Run `CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph -- <file>` to generate a flamegraph of the solution.
