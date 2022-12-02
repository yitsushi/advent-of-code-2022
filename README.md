# Advent of Code: 2022

[![codecov](https://codecov.io/gh/yitsushi/advent-of-code-2022/branch/main/graph/badge.svg)](https://codecov.io/gh/yitsushi/advent-of-code-2022)
[![Quality Check](https://github.com/yitsushi/advent-of-code-2022/actions/workflows/quality-check.yaml/badge.svg)](https://github.com/yitsushi/advent-of-code-2022/actions/workflows/quality-check.yaml)

## Run

```bash
# Run on the first part of the first day.
cargo run -- --day=1 --part=1

# Same but print timing information.
cargo run -- --day=1 --part=1 --time-it
```

## Build

```bash
# With the provided Makefile
make build

# With cargo
cargo build --release
```

## Run tests

```bash
# With the provided Makefile
make test

# With cargo
cargo test --workspace

# Run on a specific lib
cargo test -p solution

# Run on a specific day
cargo test -p solution day02
```

## Generate a new day from template

```bash
# Scaffold the 3rd Day. 
make generate_day03
```