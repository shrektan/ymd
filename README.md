
<!-- README.md is generated from README.Rmd. Please edit that file -->

# ymd

<!-- badges: start -->

[![R-CMD-check](https://github.com/shrektan/ymd/workflows/R-CMD-check/badge.svg)](https://github.com/shrektan/ymd/actions)
[![CRAN
status](https://www.r-pkg.org/badges/version/ymd)](https://CRAN.R-project.org/package=ymd)
[![Downloads from the RStudio CRAN
mirror](https://cranlogs.r-pkg.org/badges/ymd)](https://cran.r-project.org/package=ymd)
[![Rust Code
Coverage](https://coveralls.io/repos/github/shrektan/ymd/badge.svg?branch=main)](https://coveralls.io/github/shrektan/ymd?branch=main)
<!-- badges: end -->

Convert ‘YMD’ format number or string to Date efficiently, e.g.,
`211225` to `as.Date("2021-12-25")`, using Rust’s standard library. It
also provides helper functions to handle Date, e.g., quick finding the
beginning or end of the given period, adding months to Date, etc.

It’s similar to the `lubridate` package but is much lighter and focuses
only on Date objects.

## Installation

### Binary version (no Rust toolchain required)

CRAN provides the binary package. So, if you are on Windows or macOS,
the package can be installed via:

``` r
install.packages("ymd")
```

If you are on Linux, you can try to use the [RSPM (RStudio Package
Manager) repo](https://packagemanager.rstudio.com) provided by RStudio
PBC, via (remember to choose the correct binary repo URL for your
platform):

``` r
install.packages("ymd", repos = "{RSPM-Repo-URL}")
```

### Source version (Rust toolchain required)

If you want to build the dev version from source, you’ll need the Rust
toolchain, which can be installed following [the instructions from the
Rust book](https://doc.rust-lang.org/book/ch01-01-installation.html).

After that, you can build the package via:

``` r
remotes::install_github("ymd")
```

## Use Cases and Benchmarks

``` r
print_bmk <- function(x) {
  x[[1]] <- format(x[[1]])
  x[[5]] <- format(x[[5]])
  rnd <- \(v) if (is.numeric(v)) round(v, 1) else v
  x[, 1:8] |>
    lapply(rnd) |>
    as.data.frame() |>
    knitr::kable() |>
    print()
}
run_bmk <- function(..., time_unit = "us") {
  bench::mark(..., time_unit = time_unit) |> print_bmk()
}
```

### ymd

``` r
x <- c("210101", "21/02/03", "89-1-03", "1989.03.05", "01 02 03")
x <- rep(x, 100)
run_bmk(
  ymd::ymd(x),
  lubridate::ymd(x)
)
```

| expression        |    min | median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:------------------|-------:|-------:|--------:|:----------|-------:|------:|-----:|
| ymd::ymd(x)       |   39.8 |   40.6 | 24357.7 | 810.1KB   |    0.0 | 10000 |    0 |
| lubridate::ymd(x) | 1390.4 | 1546.6 |   642.9 | 9.01MB    |   10.9 |   295 |    5 |

``` r
x <- c(210101, 210224, 211231, 19890103)
x <- rep(x, 100)
run_bmk(
  ymd::ymd(x),
  lubridate::ymd(x)
)
```

| expression        |    min | median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:------------------|-------:|-------:|--------:|:----------|-------:|------:|-----:|
| ymd::ymd(x)       |   12.4 |   13.4 | 71767.6 | 3.17KB    |    0.0 | 10000 |    0 |
| lubridate::ymd(x) | 1724.9 | 1897.4 |   511.3 | 365.38KB  |   15.2 |   235 |    7 |

``` r
x <- c("2021-01-01", "2022-12-31", "1995-03-22")
x <- rep(x, 100)
run_bmk(
  ymd::ymd(x),
  lubridate::ymd(x),
  as.Date(x)
)
```

| expression        |   min | median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:------------------|------:|-------:|--------:|:----------|-------:|------:|-----:|
| ymd::ymd(x)       |  29.1 |   30.7 | 31806.3 | 2.39KB    |    3.2 |  9999 |    1 |
| lubridate::ymd(x) | 796.1 |  891.8 |  1074.0 | 193.52KB  |   15.1 |   499 |    7 |
| as.Date(x)        | 790.5 |  829.7 |  1194.6 | 85.44KB   |    0.0 |   598 |    0 |

``` r
x <- ymd::ymd(210515) + 1:100
run_bmk(
  ymd::eop$tm(x),
  lubridate::ceiling_date(x, "month") - 1
)
```

| expression                              |  min | median |  itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:----------------------------------------|-----:|-------:|---------:|:----------|-------:|------:|-----:|
| ymd::eop\$tm(x)                         |  5.7 |    6.4 | 149613.5 | 19.3KB    |    0.0 | 10000 |    0 |
| lubridate::ceiling_date(x, “month”) - 1 | 34.2 |   37.8 |  25043.7 | 155.5KB   |   27.6 |  9989 |   11 |

### edate

``` r
`%m+%` <- lubridate::`%m+%`
x <- ymd::ymd(c(200115, 200131, 200229, 200331, 200401))
x <- rep(x, 100)
run_bmk(
  ymd::edate(x, 2),
  x %m+% months(2)
)
```

| expression       |    min | median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:-----------------|-------:|-------:|--------:|:----------|-------:|------:|-----:|
| ymd::edate(x, 2) |   12.4 |   13.4 | 70780.5 | 6.2KB     |    0.0 | 10000 |    0 |
| x %m+% months(2) | 1140.7 | 1219.1 |   801.4 | 496.8KB   |    6.1 |   391 |    3 |

``` r
run_bmk(
  ymd::edate(x, -12),
  x %m+% months(-12)
)
```

| expression         |    min | median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:-------------------|-------:|-------:|--------:|:----------|-------:|------:|-----:|
| ymd::edate(x, -12) |   12.5 |   13.5 | 69090.5 | 3.95KB    |    0.0 | 10000 |    0 |
| x %m+% months(-12) | 1510.1 | 1674.2 |   589.7 | 310.64KB  |   12.7 |   278 |    6 |

### Extract Date Part

``` r
# tweak from https://github.com/Rdatatable/data.table/pull/5300
set.seed(373L)
x <- as.Date(data.table::as.IDate(sample(seq(-25000, 45000), 1e6, TRUE)))

run_bmk(
  data.table::year(x),
  lubridate::year(x),
  funchir::quick_year(x),
  ymd::year(x)
)
#> Warning: Some expressions had a GC in every iteration; so filtering is
#> disabled.
```

| expression             |      min |   median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:-----------------------|---------:|---------:|--------:|:----------|-------:|------:|-----:|
| data.table::year(x)    |   4528.2 |   4988.7 |   172.0 | 7.64MB    |   40.0 |    86 |   20 |
| lubridate::year(x)     | 281901.9 | 282685.4 |     3.5 | 57.23MB   |    7.1 |     2 |    4 |
| funchir::quick_year(x) |  30036.8 |  30385.8 |    27.7 | 26.76MB   |    7.9 |    14 |    4 |
| ymd::year(x)           |   7976.5 |   8543.1 |   113.7 | 3.82MB    |    6.0 |    57 |    3 |

``` r
run_bmk(
  data.table::month(x),
  lubridate::month(x),
  ymd::month(x)
)
#> Warning: Some expressions had a GC in every iteration; so filtering is
#> disabled.
```

| expression           |      min |   median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:---------------------|---------:|---------:|--------:|:----------|-------:|------:|-----:|
| data.table::month(x) |  21728.8 |  21979.7 |    44.8 | 7.63MB    |    3.9 |    23 |    2 |
| lubridate::month(x)  | 265033.9 | 267228.5 |     3.7 | 95.37MB   |    3.7 |     2 |    2 |
| ymd::month(x)        |   7696.7 |   8770.6 |   111.0 | 3.82MB    |    9.9 |    56 |    5 |

``` r
run_bmk(
  data.table::quarter(x),
  lubridate::quarter(x),
  ymd::quarter(x)
)
#> Warning: Some expressions had a GC in every iteration; so filtering is
#> disabled.
```

| expression             |      min |   median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:-----------------------|---------:|---------:|--------:|:----------|-------:|------:|-----:|
| data.table::quarter(x) |  17607.5 |  17977.1 |    51.1 | 7.63MB    |    7.9 |    26 |    4 |
| lubridate::quarter(x)  | 285546.4 | 289119.8 |     3.5 | 110.66MB  |    3.5 |     2 |    2 |
| ymd::quarter(x)        |  14518.9 |  15361.2 |    64.0 | 3.82MB    |    4.0 |    32 |    2 |

``` r
run_bmk(
  data.table::yday(x),
  lubridate::yday(x),
  funchir::quick_yday(x),
  ymd::yday(x)
)
#> Warning: Some expressions had a GC in every iteration; so filtering is
#> disabled.
```

| expression             |      min |   median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:-----------------------|---------:|---------:|--------:|:----------|-------:|------:|-----:|
| data.table::yday(x)    |   8542.5 |   8857.7 |   109.3 | 7.63MB    |    8.0 |    55 |    4 |
| lubridate::yday(x)     | 240595.5 | 244088.8 |     3.9 | 57.23MB   |    6.5 |     3 |    5 |
| funchir::quick_yday(x) |  23482.6 |  24758.5 |    39.7 | 19.08MB   |   13.9 |    20 |    7 |
| ymd::yday(x)           |   8091.3 |   8580.1 |   105.7 | 3.82MB    |    8.0 |    53 |    4 |

``` r
run_bmk(
  data.table::mday(x),
  lubridate::mday(x),
  funchir::quick_mday(x),
  ymd::mday(x)
)
```

| expression             |      min |   median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:-----------------------|---------:|---------:|--------:|:----------|-------:|------:|-----:|
| data.table::mday(x)    |  21263.0 |  21479.8 |    46.5 | 7.63MB    |    4.4 |    21 |    2 |
| lubridate::mday(x)     | 243894.2 | 243894.2 |     4.1 | 49.59MB   |    8.2 |     1 |    2 |
| funchir::quick_mday(x) |   9031.9 |   9400.6 |   106.0 | 15.28MB   |   16.7 |    38 |    6 |
| ymd::mday(x)           |   8168.1 |   8903.7 |   112.3 | 3.82MB    |    4.2 |    54 |    2 |

``` r
run_bmk(
  data.table::wday(x),
  lubridate::wday(x),
  ymd::wday(x)
)
```

| expression          |      min |   median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:--------------------|---------:|---------:|--------:|:----------|-------:|------:|-----:|
| data.table::wday(x) |   2781.9 |   3030.6 |   328.9 | 7.63MB    |   29.0 |   136 |   12 |
| lubridate::wday(x)  | 243859.8 | 243859.8 |     4.1 | 57.22MB   |   12.3 |     1 |    3 |
| ymd::wday(x)        |   9403.2 |  10239.4 |    97.9 | 3.82MB    |    6.5 |    45 |    3 |

``` r
run_bmk(
  data.table::isoweek(x),
  lubridate::isoweek(x),
  ymd::isoweek(x)
)
#> Warning: Some expressions had a GC in every iteration; so filtering is
#> disabled.
```

| expression             |       min |    median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:-----------------------|----------:|----------:|--------:|:----------|-------:|------:|-----:|
| data.table::isoweek(x) | 3606869.8 | 3606869.8 |     0.3 | 259.48MB  |    1.1 |     1 |    4 |
| lubridate::isoweek(x)  |  614492.8 |  614492.8 |     1.6 | 270.87MB  |    6.5 |     1 |    4 |
| ymd::isoweek(x)        |   10148.3 |   11139.3 |    87.7 | 3.82MB    |    4.0 |    44 |    2 |

## Session Info

``` r
xfun::session_info()
#> R version 4.4.1 (2024-06-14)
#> Platform: aarch64-apple-darwin20
#> Running under: macOS Sonoma 14.6
#> 
#> Locale: en_US.UTF-8 / en_US.UTF-8 / en_US.UTF-8 / C / en_US.UTF-8 / en_US.UTF-8
#> 
#> Package version:
#>   base64enc_0.1.3   bench_1.1.3       bslib_0.8.0       cachem_1.1.0     
#>   cli_3.6.3         compiler_4.4.1    cpp11_0.4.7       data.table_1.15.4
#>   digest_0.6.36     evaluate_0.24.0   fansi_1.0.6       fastmap_1.2.0    
#>   fontawesome_0.5.2 fs_1.6.4          funchir_0.2.2     generics_0.1.3   
#>   glue_1.7.0        graphics_4.4.1    grDevices_4.4.1   highr_0.11       
#>   htmltools_0.5.8.1 jquerylib_0.1.4   jsonlite_1.8.8    knitr_1.48       
#>   lifecycle_1.0.4   lubridate_1.9.3   magrittr_2.0.3    memoise_2.0.1    
#>   methods_4.4.1     mime_0.12         pillar_1.9.0      pkgconfig_2.0.3  
#>   profmem_0.6.0     R6_2.5.1          rappdirs_0.3.3    rlang_1.1.4      
#>   rmarkdown_2.27    sass_0.4.9        stats_4.4.1       tibble_3.2.1     
#>   timechange_0.3.0  tinytex_0.52      tools_4.4.1       utf8_1.2.4       
#>   utils_4.4.1       vctrs_0.6.5       xfun_0.46         yaml_2.3.10      
#>   ymd_0.1.1
```
