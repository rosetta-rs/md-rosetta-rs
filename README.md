# Rust Markdown Parsing Benchmarks

This repo tries to assess Rust markdown parsing performance.

We currently compare:
- [comrak](https://crates.io/crates/comrak)
- [mini_markdown](https://crates.io/crates/mini_markdown)
- [minimad](https://crates.io/crates/minimad)
- [pulldown-cmark](https://crates.io/crates/pulldown-cmark)

# Results

Name | Overhead (release) | Build (debug) | Parse (release) | Downloads | Version
-----|--------------------|---------------|-----------------|-----------|--------
null | 0 KiB | 720ms | 3ms | ![Download count](https://img.shields.io/crates/dr/None) | -
comrak | 2,153 KiB | 30s | 5ms | ![Download count](https://img.shields.io/crates/dr/comrak) | v0.12.1
mini_markdown | 99 KiB | 1s | N/A | ![Download count](https://img.shields.io/crates/dr/mini_markdown) | v0.2.5
minimad | 20 KiB | 1s | 2ms | ![Download count](https://img.shields.io/crates/dr/minimad) | v0.9.0
pulldown-cmark | 711 KiB | 4s | 3ms | ![Download count](https://img.shields.io/crates/dr/pulldown-cmark) | v0.9.0

*System: Linux 5.4.0-91-generic (x86_64) w/ `-j 8`*

Notes:
- Overhead will be lower if your application shares dependencies with your argument parsing library.

# Running the Benchmarks

```bash
$ ./bench.py
$ ./format.py
```
