
<!-- README.md is generated from README.Rmd. Please edit that file -->
# ymd

<!-- badges: start -->
[![R-CMD-check](https://github.com/shrektan/ymd/workflows/R-CMD-check/badge.svg)](https://github.com/shrektan/ymd/actions) <!-- badges: end -->

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
#> 1 ymd::ymd(x)         33.5   34.7    28502.  213.88KB      0  
#> 2 lubridate::ymd(x) 1887.  1935.       513.    8.22MB     19.9

x <- c(210101, 210224, 211231, 19890103)
x <- rep(x, 100)
bench::mark(
  ymd::ymd(x),
  lubridate::ymd(x), time_unit = "us"
)
#> # A tibble: 2 × 6
#>   expression           min median `itr/sec` mem_alloc `gc/sec`
#>   <bch:expr>         <dbl>  <dbl>     <dbl> <bch:byt>    <dbl>
#> 1 ymd::ymd(x)         12.2   12.7    77720.    3.17KB      0  
#> 2 lubridate::ymd(x) 1728.  1766.       561.  373.41KB     19.7

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
#> 1 ymd::ymd(x)        32.9   33.5    29587.    2.39KB      0  
#> 2 lubridate::ymd(x) 801.   824.      1203.   201.1KB     22.0
#> 3 as.Date(x)        664.   693.      1438.   87.54KB      0

x <- ymd::ymd(210515) + 1:100
bench::mark(
  ymd::eop$tm(x),
  lubridate::ceiling_date(x, "month") - 1
)
#> # A tibble: 2 × 6
#>   expression                                   min   median `itr/sec` mem_alloc
#>   <bch:expr>                              <bch:tm> <bch:tm>     <dbl> <bch:byt>
#> 1 ymd::eop$tm(x)                             5.7µs   6.15µs   158650.    19.3KB
#> 2 lubridate::ceiling_date(x, "month") - 1   96.5µs 103.53µs     9506.   255.1KB
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
#> 1 ymd::edate(x, 2)   1.07µs   1.44µs   693368.    2.24KB      0  
#> 2 x %m+% months(2) 149.85µs 160.02µs     6181.  299.23KB     45.9
bench::mark(
  ymd::edate(x, -12),
  x %m+% months(-12)
)
#> # A tibble: 2 × 6
#>   expression              min   median `itr/sec` mem_alloc `gc/sec`
#>   <bch:expr>         <bch:tm> <bch:tm>     <dbl> <bch:byt>    <dbl>
#> 1 ymd::edate(x, -12)   1.15µs   1.52µs   658489.        0B      0  
#> 2 x %m+% months(-12) 597.08µs 617.87µs     1604.    94.8KB     46.1
```
