
# ymd

<!-- badges: start -->
[![R-CMD-check](https://github.com/shrektan/ymd/workflows/R-CMD-check/badge.svg)](https://github.com/shrektan/ymd/actions)
<!-- badges: end -->

Handle common ymd Date Operations in R using Rust. It converts ymd integers or strings to Date, e.g., `211225` to `as.Date("2021-12-25")` and it provides addition helper functions like bop or eop (quick finding the begining or ending of period, e.g., the 1st date of the year or month).

It's similar to the `lubridate` package but will be much lighter and focusing on Date objects.
