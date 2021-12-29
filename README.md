# Rust Markdown Parsing Benchmarks

This repo tries to assess Rust markdown parsing performance.

We currently compare:
- [comrak](https://crates.io/crates/comrak)

# Results

Name | Overhead (release) | Build (debug) | Parse (release) | Downloads | Version
-----|--------------------|---------------|-----------------|-----------|--------
null | 0 KiB | 709ms | 2ms | ![Download count](https://img.shields.io/crates/dr/None) | -
comrak | 2,153 KiB | 30s | 5ms | ![Download count](https://img.shields.io/crates/dr/comrak) | v0.12.1

*System: Linux 5.4.0-91-generic (x86_64) w/ `-j 8`*

Notes:
- Overhead will be lower if your application shares dependencies with your argument parsing library.

# Running the Benchmarks

```bash
$ ./bench.py
$ ./format.py
```
