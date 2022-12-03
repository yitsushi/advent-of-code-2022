# Advent of Code: 2022

[![codecov](https://codecov.io/gh/yitsushi/advent-of-code-2022/branch/main/graph/badge.svg)](https://codecov.io/gh/yitsushi/advent-of-code-2022)
[![Quality Check](https://github.com/yitsushi/advent-of-code-2022/actions/workflows/quality-check.yaml/badge.svg)](https://github.com/yitsushi/advent-of-code-2022/actions/workflows/quality-check.yaml)

First of all, I have no idea what I'm doing. I like challenges and doing Advent
of Code using a language I barely know, well that's the perfect challenge and
opportunity to learn. The commit history can be a good journal about my journey
with rust, however don't take any of this as a good practice. I really don't know
how a rust developer works in the wild, this project structure is totally an
opinionated structure, and the produced code is definitely not a professional
rust code, don't copy it and take it as "that's how to do it". Technically you
can copy it and I have no objections if you understand **I take zero
responsibilities if you break anything in production**, or you get a lot of
down votes on StackOverflow because you copied something that shouldn't exist.

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