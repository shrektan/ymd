# R CMD check results

0 errors | 0 warnings | 1 note

* Fix the bug in the rust code that causes the `period_begin()` function
  to fail when the input is an integer vector of dates.

## Note 1

* checking installed package size ... NOTE
  installed size is  7.7Mb
  sub-directories of 1Mb or more:
    libs   7.6Mb

The size of the package can't be reduced further as it has to bundle
all the Rust cates dependencies to avoid the downloading during
installation issue.
