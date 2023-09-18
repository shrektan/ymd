## R CMD check results

0 errors | 0 warnings | 2 note

* This is a new release.

### Note 1
* checking CRAN incoming feasibility ... NOTE
  Maintainer: ‘Xianying Tan <shrektan@126.com>’

  New submission

  Package was archived on CRAN

  CRAN repository db overrides:
    X-CRAN-Comment: Archived on 2023-08-19 for policy violation.

    Downloading on installation from github.

The package was archived on CRAN because it downloads rust dependencies during
installing and use multiple CPUs when building rust binaries. I've fixed the two issues.

### Note 2

* checking installed package size ... NOTE
  installed size is  7.5Mb
  sub-directories of 1Mb or more:
    libs   7.5Mb

The reason that the package size is large is it includes the Rust dependencies.
