# R CMD check results

0 errors | 0 warnings | 0 notes

Tested on:

* macOS (local), R 4.6.1

The exact source tarball checked locally was also accepted for checking by
win-builder R-devel; its result is pending. The official macOS builder
currently offers only R-devel, and two upload attempts returned HTTP 502.

# CRAN check issue

This release fixes the compiled-code NOTE about the non-API
`R_NamespaceRegistry` entry point by upgrading the extendr Rust crates to
version 0.9.0. The installed shared library was also checked directly and no
longer contains this symbol.

The installation path was additionally updated so that the committed
Rust-generated R wrappers are not regenerated during a normal package install.
This avoids a second host/target Cargo build; inspection of the exact candidate
installation log confirmed one Rust library build and no wrapper build.

The vendored Rust dependencies are packaged in a deterministic archive without
macOS extended attributes. Installation remains fully offline, and extracting
the exact candidate locally emitted no extended-header messages.

This release additionally fixes handling of fractional `Date` values. Values
are floored consistently with base R, including before the Unix epoch, while
non-finite and out-of-range values return missing results.

# Reverse dependencies

I checked both current direct reverse dependencies, `fcl` and `fastymd`, using
this release and compared their results with the current CRAN release of `ymd`.
There were no new errors, warnings, notes, or test failures. `fastymd` passed
both checks. `fcl` reported the same pre-existing compiled-code NOTE about its
own `R_NamespaceRegistry` reference with both versions of `ymd`.
