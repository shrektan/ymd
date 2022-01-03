
<!-- README.md is generated from README.Rmd. Please edit that file -->
# ymd

<!-- badges: start -->
[![R-CMD-check](https://github.com/shrektan/ymd/workflows/R-CMD-check/badge.svg)](https://github.com/shrektan/ymd/actions) <!-- badges: end -->

Handle common ymd Date Operations in R using Rust. It converts ymd integers or strings to Date, e.g., `211225` to `as.Date("2021-12-25")` and it provides addition helper functions like bop or eop (quick finding the begining or ending of period, e.g., the 1st date of the year or month).

It's similar to the `lubridate` package but will be much lighter and focusing on Date objects.

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
#> 1 ymd::ymd(x)         32.7   33.5    29387.   209.5KB      0  
#> 2 lubridate::ymd(x) 1849.  1929.       514.     8.2MB     19.9

x <- c(210101, 210224, 211231, 19890103)
x <- rep(x, 100)
bench::mark(
  ymd::ymd(x),
  lubridate::ymd(x), time_unit = "us"
)
#> # A tibble: 2 × 6
#>   expression           min median `itr/sec` mem_alloc `gc/sec`
#>   <bch:expr>         <dbl>  <dbl>     <dbl> <bch:byt>    <dbl>
#> 1 ymd::ymd(x)         12.1   12.7    77143.    3.17KB      0  
#> 2 lubridate::ymd(x) 1681.  1735.       566.  373.41KB     19.6

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
#> 1 ymd::ymd(x)        32.0   33.3    29191.    2.39KB     0   
#> 2 lubridate::ymd(x) 787.   842.      1158.   201.1KB    19.7 
#> 3 as.Date(x)        665.   678.      1460.   87.54KB     2.02
```
