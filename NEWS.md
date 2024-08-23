# ymd 0.1.2

* Added rustc version reporting in the installation log as per CRAN policy.

# ymd 0.1.1

* Fix the calling non-API entry points issue.

# ymd 0.1.0

* `ymd()` now supports `...` arguments, which is convenient for interactive use, e.g., `ymd(210101, 220201)`.
* Implement date part extracting functions, including `year()`, `month()`, `quarter()`, `isoweek()`, `isowday()`, `wday()`, `mday()`, `yday()`,
* Redirect the Rust message to R's stderr stream.
* Fix issues that violate the CRAN policies, e.g., license issues and rust offline dependencies.

# ymd 0.0.1

* Added a `NEWS.md` file to track changes to the package.
* Implement `ymd()`, `bop$xx()`, `eop$xx()` and `edate()` in Rust.
