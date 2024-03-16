# 1brc - One Billion Row Challenge

This is a rust solution for the One Billion Row Challenge.

## Init

Run `bash init.sh` to clone the original repository of the challenge and generate the data, then move it to our directory.

## Run

Run `cargo run` to solve the challenge.

## Benchmarking

You can benchmark the solution with cargo-flamegraph.
Run `CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph -- <file>` to generate a flamegraph of the solution.

## Debugging

To check if ranges are overlapping, use the following command.

```bash
./target/debug/process measurements_e3.txt 8 | sort --key=3n -r | sort --key=2n --numeric-sort -r | awk '{ print substr($0, index($0,$4)) }' | xargs -I {} grep -n {} measurements_e3.txt | cut -d ":" -f 1
```

Print the duplicated lines, that are processed by multiple threads.

```bash
./target/debug/process measurements_e3.txt 8 | sort --key=3n -r | sort --key=2n --numeric-sort -r | awk '{ print substr($0, index($0,$4)) }' | xargs -I {} grep -n {} measurements_e3.txt | cut -d ":" -f 1 | uniq -d
```

# TODO

- [x] Split the file reading in chunks and process each chunk in parallel.
- [ ] Try to optimize parsing.
- [ ] SIMD
