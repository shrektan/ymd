# R CMD check results

0 errors | 0 warnings | 2 note

* Added rustc version reporting in the installation log as per CRAN policy.

## Note 1

* checking installed package size ... NOTE
  installed size is  7.7Mb
  sub-directories of 1Mb or more:
    libs   7.6Mb

The size of the package can't be reduced further as it has to bundle
all the Rust cates dependencies to avoid the downloading during
installation issue.

## Note2

* checking CRAN incoming feasibility ... [12s] NOTE
Maintainer: 'Xianying Tan <shrektan@126.com>'

Days since last update: 6

This recent update is to address CRAN's request to report the rustc version
in the installation log, as per the policy for Rust packages on CRAN.
