# Rust Markdown Parsing Benchmarks

This repo tries to assess Rust markdown parsing performance.

We currently compare:
- [comrak](https://crates.io/crates/comrak)
- [minimad](https://crates.io/crates/minimad)
- [pulldown-cmark](https://crates.io/crates/pulldown-cmark)
- [markdown](https://crates.io/crates/markdown)

Formerly, we compared:
- [mini_markdown](https://crates.io/crates/mini_markdown): [author is unwilling to follow cargo's variant of semver](https://github.com/darakian/mini_markdown/issues/60)

Suggestions:
- Generally, `pulldown-cmark`

# Results

Name | Overhead (release) | Build (debug) | Parse (release) | Downloads | Version
-----|--------------------|---------------|-----------------|-----------|--------
null | 0 KiB | 216ms | 2ms | - | -
comrak | 583 KiB | 17s | 5ms | ![Download count](https://img.shields.io/crates/dr/comrak) | v0.32.0
jotdown | 209 KiB | 1s | 4ms | ![Download count](https://img.shields.io/crates/dr/jotdown) | v0.7.0
markdown | 985 KiB | 1s | 10ms | ![Download count](https://img.shields.io/crates/dr/markdown) | v1.0.0-alpha.21
minimad | 26 KiB | 569ms | 3ms | ![Download count](https://img.shields.io/crates/dr/minimad) | v0.13.1
pulldown-cmark | 445 KiB | 2s | 3ms | ![Download count](https://img.shields.io/crates/dr/pulldown-cmark) | v0.12.2

*System: Linux 5.4.0-170-generic (x86_64), rustc 1.84.0 (9fc6b4312 2025-01-07) w/ `-j 8`*

Notes:
- Overhead will be lower if your application shares dependencies with your argument parsing library.

# Running the Benchmarks

```bash
$ ./bench.py
$ ./format.py
```
