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
null | 0 KiB | 222ms | 4ms | - | -
comrak | 549 KiB | 17s | 3ms | ![Download count](https://img.shields.io/crates/dr/comrak) | v0.21.0
markdown | 931 KiB | 1s | 10ms | ![Download count](https://img.shields.io/crates/dr/markdown) | v1.0.0-alpha.16
minimad | 26 KiB | 603ms | 4ms | ![Download count](https://img.shields.io/crates/dr/minimad) | v0.13.0
pulldown-cmark | 708 KiB | 1s | 3ms | ![Download count](https://img.shields.io/crates/dr/pulldown-cmark) | v0.9.6

*System: Linux 5.4.0-124-generic (x86_64), rustc 1.75.0 (82e1608df 2023-12-21) w/ `-j 8`*

Notes:
- Overhead will be lower if your application shares dependencies with your argument parsing library.

# Running the Benchmarks

```bash
$ ./bench.py
$ ./format.py
```
