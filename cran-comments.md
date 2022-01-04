## Test environments

* local macOS install: release
* GitHub Actions CI (Windows, macOS, Linux): release and devel, ucrt-devel
* win-builder: release and devel
* R Mac builder
* R-hub builder for CRAN

## R CMD check results

0 errors | 0 warnings | 2 note

* checking CRAN incoming feasibility ... NOTE
Maintainer: 'Xianying Tan <shrektan@126.com>'

New submission

* checking installed package size ... NOTE
  installed size is  6.3Mb
  sub-directories of 1Mb or more:
    libs   6.2Mb

The above NOTE about size only happens on R Mac builder and one of the Ubuntu machine of R-hub builder.

## My comments

* This is my new submission.

* I would like to request to exclude Solaris from the build targets because
  Solaris is not a supported platform by Rust. This should be in line with the
  treatments of other CRAN packages that use Rust; gifski, baseflow, salso,
  string2path are not built on Solaris.

## Downstream dependencies

* There's no reverse dependency.
