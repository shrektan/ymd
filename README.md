
<!-- README.md is generated from README.Rmd. Please edit that file -->
# ymd

<!-- badges: start -->
[![R-CMD-check](https://github.com/shrektan/ymd/workflows/R-CMD-check/badge.svg)](https://github.com/shrektan/ymd/actions) <!-- badges: end -->

Handle common ymd Date Operations in R using Rust. It converts ymd integers or strings to Date, e.g., `211225` to `as.Date("2021-12-25")` and it provides addition helper functions like bop or eop (quick finding the begining or ending of period, e.g., the 1st date of the year or month).

It's similar to the `lubridate` package but will be much lighter and focuses on Date objects.

## Installation

Note, in order to compile this package from source you need the Rust toolchain, which can be found in [Rust's official website](https://www.rust-lang.org).

## Some use case and benchmarks

``` r
x <- c("210101", "21/02/03", "89-1-03", "1989.03.05")
x <- rep(x, 100)
bench::mark(
  ymd::ymd(x),
  lubridate::ymd(x), time_unit = "us"
)
#> # A tibble: 2 × 6
#>   expression           min median `itr/sec` mem_alloc `gc/sec`
#>   <bch:expr>         <dbl>  <dbl>     <dbl> <bch:byt>    <dbl>
#> 1 ymd::ymd(x)         33.0   34.1    29096.   209.5KB      0  
#> 2 lubridate::ymd(x) 1842.  1906.       517.     8.2MB     19.9

x <- c(210101, 210224, 211231, 19890103)
x <- rep(x, 100)
bench::mark(
  ymd::ymd(x),
  lubridate::ymd(x), time_unit = "us"
)
#> # A tibble: 2 × 6
#>   expression           min median `itr/sec` mem_alloc `gc/sec`
#>   <bch:expr>         <dbl>  <dbl>     <dbl> <bch:byt>    <dbl>
#> 1 ymd::ymd(x)         11.8   12.3    79154.    3.17KB      0  
#> 2 lubridate::ymd(x) 1679.  1709.       583.  373.41KB     22.0

x <- c("2021-01-01", "2022-12-31", "1995-03-22")
x <- rep(x, 100)
bench::mark(
  ymd::ymd(x),
  lubridate::ymd(x), time_unit = "us",
  as.Date(x)
)
#> # A tibble: 3 × 6
#>   expression          min median `itr/sec` mem_alloc `gc/sec`
#>   <bch:expr>        <dbl>  <dbl>     <dbl> <bch:byt>    <dbl>
#> 1 ymd::ymd(x)        32.1   33.1    29449.    2.39KB     2.95
#> 2 lubridate::ymd(x) 782.   821.      1198.   201.1KB    19.7 
#> 3 as.Date(x)        662.   678.      1462.   87.54KB     2.02

x <- ymd::ymd(210515) + 1:100
bench::mark(
  ymd::eop$tm(x),
  lubridate::ceiling_date(x, "month") - 1
)
#> # A tibble: 2 × 6
#>   expression                                   min   median `itr/sec` mem_alloc
#>   <bch:expr>                              <bch:tm> <bch:tm>     <dbl> <bch:byt>
#> 1 ymd::eop$tm(x)                            5.54µs   6.19µs   159218.    19.2KB
#> 2 lubridate::ceiling_date(x, "month") - 1  94.87µs 101.52µs     9594.   255.1KB
#> # … with 1 more variable: gc/sec <dbl>

`%m+%` <- lubridate::`%m+%`
x <- ymd::ymd(c(200115, 200131, 200229, 200331, 200401))
bench::mark(
  ymd::edate(x, 2),
  x %m+% months(2)
)
#> # A tibble: 2 × 6
#>   expression            min   median `itr/sec` mem_alloc `gc/sec`
#>   <bch:expr>       <bch:tm> <bch:tm>     <dbl> <bch:byt>    <dbl>
#> 1 ymd::edate(x, 2)   1.07µs   1.39µs   652474.    2.24KB      0  
#> 2 x %m+% months(2) 146.21µs 155.23µs     6307.  299.23KB     48.1
bench::mark(
  ymd::edate(x, -12),
  x %m+% months(-12)
)
#> # A tibble: 2 × 6
#>   expression              min   median `itr/sec` mem_alloc `gc/sec`
#>   <bch:expr>         <bch:tm> <bch:tm>     <dbl> <bch:byt>    <dbl>
#> 1 ymd::edate(x, -12)   1.19µs   1.56µs   627858.        0B      0  
#> 2 x %m+% months(-12) 573.47µs 591.79µs     1666.    94.8KB     48.2
```
