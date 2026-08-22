# Changelog

## ymd 0.1.6

- Fix the CRAN compiled-code NOTE about the non-API
  `R_NamespaceRegistry` entry point by upgrading the extendr Rust crates
  to version 0.9.0.
- Build bundled Rust dependencies offline and refresh their authorship
  and license metadata.
- Add Rust target selection and continuous-integration coverage for
  Windows ARM64.
- Avoid rebuilding the Rust crate to generate wrappers during normal
  package installation; wrapper generation now runs only during package
  documentation.

## ymd 0.1.5

CRAN release: 2025-04-15

- Fix the calling non-API entry points issue by upgrading the rust
  crates.

## ymd 0.1.4

CRAN release: 2024-11-26

- Fix the issue that `period_begin()` function returns only the first
  element for integer vectors, like `c(20240101, 20240102)`.

## ymd 0.1.3

CRAN release: 2024-11-06

- Fix the bug inside of `period_begin()` function to support both
  integer and double dates.

## ymd 0.1.2

CRAN release: 2024-08-23

- Added rustc version reporting in the installation log as per CRAN
  policy.

## ymd 0.1.1

CRAN release: 2024-08-17

- Fix the calling non-API entry points issue.

## ymd 0.1.0

CRAN release: 2023-10-09

- [`ymd()`](https://shrektan.github.io/ymd/reference/ymd.md) now
  supports `...` arguments, which is convenient for interactive use,
  e.g., `ymd(210101, 220201)`.
- Implement date part extracting functions, including
  [`year()`](https://shrektan.github.io/ymd/reference/date_part.md),
  [`month()`](https://shrektan.github.io/ymd/reference/date_part.md),
  [`quarter()`](https://shrektan.github.io/ymd/reference/date_part.md),
  [`isoweek()`](https://shrektan.github.io/ymd/reference/date_part.md),
  [`isowday()`](https://shrektan.github.io/ymd/reference/date_part.md),
  [`wday()`](https://shrektan.github.io/ymd/reference/date_part.md),
  [`mday()`](https://shrektan.github.io/ymd/reference/date_part.md),
  [`yday()`](https://shrektan.github.io/ymd/reference/date_part.md),
- Redirect the Rust message to R’s stderr stream.
- Fix issues that violate the CRAN policies, e.g., license issues and
  rust offline dependencies.

## ymd 0.0.1

CRAN release: 2022-01-06

- Added a `NEWS.md` file to track changes to the package.
- Implement [`ymd()`](https://shrektan.github.io/ymd/reference/ymd.md),
  `bop$xx()`, `eop$xx()` and
  [`edate()`](https://shrektan.github.io/ymd/reference/edate.md) in
  Rust.
