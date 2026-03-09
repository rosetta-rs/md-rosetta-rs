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
null | 0 KiB | 193ms | 1ms | - | -
comrak | 2,767 KiB | 14s | 2ms | ![Download count](https://img.shields.io/crates/dr/comrak) | v0.50.0
jotdown | 222 KiB | 1s | 2ms | ![Download count](https://img.shields.io/crates/dr/jotdown) | v0.9.1
markdown | 1,026 KiB | 1s | 11ms | ![Download count](https://img.shields.io/crates/dr/markdown) | v1.0.0
minimad | 29 KiB | 544ms | 2ms | ![Download count](https://img.shields.io/crates/dr/minimad) | v0.14.0
pulldown-cmark | 440 KiB | 2s | 2ms | ![Download count](https://img.shields.io/crates/dr/pulldown-cmark) | v0.13.1

*System: Linux 6.17.9-76061709-generic (x86_64), rustc 1.94.0 (4a4ef493e 2026-03-02) w/ `-j 8`*

Notes:
- Overhead will be lower if your application shares dependencies with your argument parsing library.

# Running the Benchmarks

```bash
$ ./bench.py
$ ./format.py
```
