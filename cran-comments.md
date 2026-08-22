# R CMD check results

0 errors | 0 warnings | 0 notes

Tested on:

* macOS (local), R 4.5.1
* macOS, R-release 4.6.1 (GitHub Actions)
* Windows x86_64, R-devel 4.7.0 (2026-08-21 r90440, win-builder)
* Windows x86_64, R-release 4.6.1 and R-devel 4.7.0 (GitHub Actions)
* Windows ARM64, R-release 4.6.1 (GitHub Actions)
* Ubuntu, R-release 4.6.1 and R-devel 4.7.0 (GitHub Actions)

# CRAN check issue

This release fixes the compiled-code NOTE about the non-API
`R_NamespaceRegistry` entry point by upgrading the extendr Rust crates to
version 0.9.0. The installed shared library was also checked directly and no
longer contains this symbol.

The Windows installation path was additionally updated so that the committed
Rust-generated R wrappers are not regenerated during a normal package install.
This avoids a second host/target Cargo build; the official R-devel win-builder
check now completes with `Status: OK`.

# Reverse dependencies

I checked both current direct reverse dependencies, `fcl` and `fastymd`, using
this release and compared their results with the current CRAN release of `ymd`.
There were no new errors, warnings, notes, or test failures.
