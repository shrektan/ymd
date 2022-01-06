
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
print_bmk <- function(x) {
  x[[1]] <- format(x[[1]])
  x[[5]] <- format(x[[5]])
  rnd <- \(v) if (is.numeric(v)) round(v, 1) else v
  x[, 1:9] |> lapply(rnd) |> as.data.frame() |> knitr::kable() |> print()
}
x <- c("210101", "21/02/03", "89-1-03", "1989.03.05")
x <- rep(x, 100)
bench::mark(
  ymd::ymd(x),
  lubridate::ymd(x), time_unit = "us"
) |> print_bmk()
```

| expression        |     min|  median|  itr.sec| mem\_alloc |  gc.sec|  n\_itr|  n\_gc|  total\_time|
|:------------------|-------:|-------:|--------:|:-----------|-------:|-------:|------:|------------:|
| ymd::ymd(x)       |    32.8|    33.7|  29107.3| 214.27KB   |     0.0|   10000|      0|     343556.5|
| lubridate::ymd(x) |  1843.0|  1932.1|    511.4| 8.19MB     |    19.9|     231|      9|     451659.5|

``` r

x <- c(210101, 210224, 211231, 19890103)
x <- rep(x, 100)
bench::mark(
  ymd::ymd(x),
  lubridate::ymd(x), time_unit = "us"
) |> print_bmk()
```

| expression        |     min|  median|  itr.sec| mem\_alloc |  gc.sec|  n\_itr|  n\_gc|  total\_time|
|:------------------|-------:|-------:|--------:|:-----------|-------:|-------:|------:|------------:|
| ymd::ymd(x)       |    11.8|    12.4|  78836.5| 3.17KB     |     0.0|   10000|      0|     126844.8|
| lubridate::ymd(x) |  1707.7|  1843.4|    527.9| 373.41KB   |    19.8|     240|      9|     454630.2|

``` r

x <- c("2021-01-01", "2022-12-31", "1995-03-22")
x <- rep(x, 100)
bench::mark(
  ymd::ymd(x),
  lubridate::ymd(x), time_unit = "us",
  as.Date(x)
) |> print_bmk()
```

| expression        |    min|  median|  itr.sec| mem\_alloc |  gc.sec|  n\_itr|  n\_gc|  total\_time|
|:------------------|------:|-------:|--------:|:-----------|-------:|-------:|------:|------------:|
| ymd::ymd(x)       |   32.0|    32.6|  30097.2| 2.39KB     |     0.0|   10000|      0|     332257.2|
| lubridate::ymd(x) |  787.2|   837.2|   1172.8| 201.1KB    |    19.6|     539|      9|     459576.8|
| as.Date(x)        |  659.6|   690.4|   1435.8| 87.54KB    |     2.0|     711|      1|     495201.9|

``` r

x <- ymd::ymd(210515) + 1:100
bench::mark(
  ymd::eop$tm(x),
  lubridate::ceiling_date(x, "month") - 1, time_unit = "us"
) |> print_bmk()
```

| expression                               |   min|  median|   itr.sec| mem\_alloc |  gc.sec|  n\_itr|  n\_gc|  total\_time|
|:-----------------------------------------|-----:|-------:|---------:|:-----------|-------:|-------:|------:|------------:|
| ymd::eop$tm(x)                           |   5.5|     5.9|  164290.1| 19.3KB     |     0.0|   10000|      0|      60867.9|
| lubridate::ceiling\_date(x, "month") - 1 |  94.5|   101.4|    9547.0| 255.1KB    |    35.6|    4285|     16|     448832.8|

``` r

`%m+%` <- lubridate::`%m+%`
x <- ymd::ymd(c(200115, 200131, 200229, 200331, 200401))
x <- rep(x, 100)
bench::mark(
  ymd::edate(x, 2),
  x %m+% months(2), time_unit = "us"
) |> print_bmk()
```

| expression       |    min|  median|  itr.sec| mem\_alloc |  gc.sec|  n\_itr|  n\_gc|  total\_time|
|:-----------------|------:|-------:|--------:|:-----------|-------:|-------:|------:|------------:|
| ymd::edate(x, 2) |   13.2|    13.8|  70377.8| 6.2KB      |     0.0|   10000|      0|     142090.2|
| x %m+% months(2) |  293.6|   309.5|   3197.9| 424.6KB    |    23.7|    1485|     11|     464365.5|

``` r
bench::mark(
  ymd::edate(x, -12),
  x %m+% months(-12), time_unit = "us"
) |> print_bmk()
```

| expression         |    min|  median|  itr.sec| mem\_alloc |  gc.sec|  n\_itr|  n\_gc|  total\_time|
|:-------------------|------:|-------:|--------:|:-----------|-------:|-------:|------:|------------:|
| ymd::edate(x, -12) |   13.3|    14.0|  70184.0| 3.95KB     |     0.0|   10000|      0|     142482.6|
| x %m+% months(-12) |  793.4|   829.1|   1189.7| 317.19KB   |    32.7|     509|     14|     427843.8|
