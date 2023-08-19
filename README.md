# Wordle Solver

> Powered by Readrock SRE

usage:

```bash
cargo run --release
```

## Reference

- https://github.com/mahavivo/english-wordlists/blob/master/SUM_of_cet4%2B6%2Btoefl%2Bgre.txt

## Conduct

```rust
// 0: not match
// 1: match, but not in the same position
// 2: match, and in the same position
// pattern: ternary arithmetic
```