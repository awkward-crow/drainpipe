# drainpipe

[![ci](https://github.com/awkward-crow/drainpipe/actions/workflows/ci.yml/badge.svg)](https://github.com/awkward-crow/drainpipe/actions/workflows/ci.yml)

Rust implementations of the programming styles described in *Exercises in Programming Style* by Cristina Videira Lopes ([source](https://github.com/crista/exercises-in-programming-style)).

Each subdirectory contains a self-contained implementation of the term-frequency problem in a distinct style, structured as a Cargo workspace.

## styles

| directory | style |
|---|---|
| `pipeline` | functions arranged as a linear pipeline |
| `things` | object-oriented, objects with methods |
| `golf` | code golf — minimal, idiomatic Rust |
| `infinite_mirror` | recursion |
| `norse` | persistent tables using Polars dataframes |
| `drainpipe` | optimised — HashSet for stop words, no regex |

## usage

Run any implementation from the workspace root, for example:

```sh
cargo run -p pipeline -- pride-and-prejudice.txt
```

## performance

The `drainpipe` package trades regex for an explicit character scan and uses a `HashSet` for stop word lookup, bringing the runtime on `pride-and-prejudice.txt` from ~17ms down to ~7ms:

```sh
hyperfine --warmup=2 "./target/release/drainpipe pride-and-prejudice.txt"
```

```
Benchmark 1: ./target/release/drainpipe pride-and-prejudice.txt
  Time (mean ± σ):       7.1 ms ±   0.9 ms    [User: 7.0 ms, System: 0.4 ms]
  Range (min … max):     6.2 ms …  14.6 ms    348 runs
```
