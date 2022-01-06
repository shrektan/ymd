
<!-- README.md is generated from README.Rmd. Please edit that file -->
# ymd

<!-- badges: start -->
[![R-CMD-check](https://github.com/shrektan/ymd/workflows/R-CMD-check/badge.svg)](https://github.com/shrektan/ymd/actions) [![CRAN status](https://www.r-pkg.org/badges/version/ymd)](https://CRAN.R-project.org/package=ymd) [![Downloads from the RStudio CRAN mirror](https://cranlogs.r-pkg.org/badges/ymd)](https://cran.r-project.org/package=ymd) <!-- badges: end -->

Convert 'YMD' format number or string to Date efficiently, e.g., `211225` to `as.Date("2021-12-25")`, using Rust's standard library. It also provides helper functions to handle Date, e.g., quick finding the beginning or ending of the given period, adding months to Date, etc.

It's similar to the `lubridate` package but is much lighter and focuses only on Date objects.

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
#> 1 ymd::ymd(x)         32.8   33.9    28913.  214.46KB      0  
#> 2 lubridate::ymd(x) 1841.  1887.       522.    8.21MB     19.8

x <- c(210101, 210224, 211231, 19890103)
x <- rep(x, 100)
bench::mark(
  ymd::ymd(x),
  lubridate::ymd(x), time_unit = "us"
)
#> # A tibble: 2 × 6
#>   expression           min median `itr/sec` mem_alloc `gc/sec`
#>   <bch:expr>         <dbl>  <dbl>     <dbl> <bch:byt>    <dbl>
#> 1 ymd::ymd(x)         11.8   12.3    79266.    3.17KB      0  
#> 2 lubridate::ymd(x) 1681.  1715.       581.  373.41KB     21.9

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
#> 1 ymd::ymd(x)        31.9   32.7    30444.    2.39KB      0  
#> 2 lubridate::ymd(x) 784.   800.      1241.   201.1KB     21.9
#> 3 as.Date(x)        663.   674.      1475.   87.54KB      0

x <- ymd::ymd(210515) + 1:100
bench::mark(
  ymd::eop$tm(x),
  lubridate::ceiling_date(x, "month") - 1
)
#> # A tibble: 2 × 6
#>   expression                                   min   median `itr/sec` mem_alloc
#>   <bch:expr>                              <bch:tm> <bch:tm>     <dbl> <bch:byt>
#> 1 ymd::eop$tm(x)                            5.45µs    5.9µs   159295.    19.3KB
#> 2 lubridate::ceiling_date(x, "month") - 1  95.28µs  102.2µs     9579.   255.1KB
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
#> 1 ymd::edate(x, 2)   1.11µs   1.39µs   691437.    2.24KB      0  
#> 2 x %m+% months(2) 146.49µs 160.88µs     6021.  299.23KB     46.2
bench::mark(
  ymd::edate(x, -12),
  x %m+% months(-12)
)
#> # A tibble: 2 × 6
#>   expression              min   median `itr/sec` mem_alloc `gc/sec`
#>   <bch:expr>         <bch:tm> <bch:tm>     <dbl> <bch:byt>    <dbl>
#> 1 ymd::edate(x, -12)   1.11µs   1.52µs   622982.        0B      0  
#> 2 x %m+% months(-12)  576.3µs 615.62µs     1566.    94.8KB     46.2
```
