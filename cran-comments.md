# R CMD check results

0 errors | 0 warnings | 2 note

* This is a new release.

## Note 1

* checking CRAN incoming feasibility ... NOTE
Maintainer: 'Xianying Tan <shrektan@126.com>'

New submission

Package was archived on CRAN

CRAN repository db overrides:
  X-CRAN-Comment: Archived on 2024-07-31 as check problems were not
    corrected despite reminders.

The package was archived on CRAN due to the underlying Rust code
calling non-API entry points in R. This issue has now been resolved.
We apologize for not addressing this before the July 31, 2024 deadline.

## Note 2

* checking installed package size ... NOTE
  installed size is  7.7Mb
  sub-directories of 1Mb or more:
    libs   7.6Mb

The size of the package can't be reduced further as it has to bundle
all the Rust cates dependencies to avoid the downloading during
installation issue.
