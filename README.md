
<!-- README.md is generated  from README.Rmd. Please edit that file -->

# ymd

<!-- badges: start -->

[![R-CMD-check](https://github.com/shrektan/ymd/workflows/R-CMD-check/badge.svg)](https://github.com/shrektan/ymd/actions)
[![CRAN
status](https://www.r-pkg.org/badges/version/ymd)](https://CRAN.R-project.org/package=ymd)
[![Downloads from the RStudio CRAN
mirror](https://cranlogs.r-pkg.org/badges/ymd)](https://cran.r-project.org/package=ymd)
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
| ymd::ymd(x)       |   41.8 |   42.9 | 22733.0 | 810.52KB  |    2.3 |  9999 |    1 |
| lubridate::ymd(x) | 1364.7 | 1494.1 |   647.3 | 9.11MB    |   13.0 |   298 |    6 |

``` r

x <- c(210101, 210224, 211231, 19890103)
x <- rep(x, 100)
run_bmk(
  ymd::ymd(x),
  lubridate::ymd(x)
)
```

| expression        |    min | median |  itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:------------------|-------:|-------:|---------:|:----------|-------:|------:|-----:|
| ymd::ymd(x)       |    4.1 |    4.8 | 171097.1 | 3.17KB    |    0.0 | 10000 |    0 |
| lubridate::ymd(x) | 1690.1 | 1974.2 |    492.7 | 365.38KB  |   15.1 |   228 |    7 |

``` r

x <- c("2021-01-01", "2022-12-31", "1995-03-22")
x <- rep(x, 100)
run_bmk(
  ymd::ymd(x),
  clock::as_date(clock::year_month_day_parse(x)),
  lubridate::ymd(x),
  as.Date(x)
)
```

| expression | min | median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:---|---:|---:|---:|:---|---:|---:|---:|
| ymd::ymd(x) | 30.5 | 31.6 | 30648.7 | 2.39KB | 0.0 | 10000 | 0 |
| clock::as_date(clock::year_month_day_parse(x)) | 159.7 | 167.0 | 5590.3 | 3.66MB | 21.8 | 2566 | 10 |
| lubridate::ymd(x) | 787.4 | 855.8 | 1104.3 | 193.52KB | 17.0 | 519 | 8 |
| as.Date(x) | 1051.0 | 1081.9 | 902.8 | 101.69KB | 0.0 | 452 | 0 |

``` r

x <- ymd::ymd(210515) + 1:100
run_bmk(
  ymd::eop$tm(x),
  lubridate::ceiling_date(x, "month") - 1
)
```

| expression | min | median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:---|---:|---:|---:|:---|---:|---:|---:|
| ymd::eop\$tm(x) | 5.8 | 6.2 | 149777.9 | 19.3KB | 15.0 | 9999 | 1 |
| lubridate::ceiling_date(x, “month”) - 1 | 35.5 | 37.7 | 24565.2 | 159.5KB | 29.5 | 9988 | 12 |

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
| ymd::edate(x, 2) |   12.3 |   13.0 | 73250.4 | 6.2KB     |    7.3 |  9999 |    1 |
| x %m+% months(2) | 1344.3 | 1438.6 |   682.7 | 496.5KB   |    4.1 |   337 |    2 |

``` r
run_bmk(
  ymd::edate(x, -12),
  x %m+% months(-12)
)
```

| expression         |    min | median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:-------------------|-------:|-------:|--------:|:----------|-------:|------:|-----:|
| ymd::edate(x, -12) |   12.4 |   13.2 | 72652.5 | 3.95KB    |    7.3 |  9999 |    1 |
| x %m+% months(-12) | 1758.2 | 1871.2 |   517.6 | 310.65KB  |   10.4 |   248 |    5 |

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
| data.table::year(x)    |   3357.6 |   3477.4 |   271.4 | 7.64MB    |   35.9 |   136 |   18 |
| lubridate::year(x)     | 206448.2 | 239828.0 |     4.4 | 64.85MB   |    8.7 |     3 |    6 |
| funchir::quick_year(x) |  26009.0 |  26505.6 |    29.6 | 22.93MB   |   13.8 |    15 |    7 |
| ymd::year(x)           |   6800.3 |   6992.0 |   140.9 | 3.82MB    |    7.9 |    71 |    4 |

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
| data.table::month(x) |  20128.4 |  20535.7 |    48.1 | 7.63MB    |    9.6 |    25 |    5 |
| lubridate::month(x)  | 256228.7 | 258042.5 |     3.9 | 103MB     |    7.8 |     2 |    4 |
| ymd::month(x)        |   7236.2 |   7411.4 |   132.1 | 3.82MB    |    7.9 |    67 |    4 |

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
| data.table::quarter(x) |  16649.2 |  17041.2 |    58.1 | 7.63MB    |    5.8 |    30 |    3 |
| lubridate::quarter(x)  | 278270.2 | 280509.0 |     3.6 | 118.29MB  |   10.7 |     2 |    6 |
| ymd::quarter(x)        |  13623.7 |  13850.3 |    71.7 | 3.82MB    |    4.0 |    36 |    2 |

``` r
run_bmk(
  data.table::yday(x),
  lubridate::yday(x),
  funchir::quick_yday(x),
  ymd::yday(x)
)
```

| expression             |      min |   median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:-----------------------|---------:|---------:|--------:|:----------|-------:|------:|-----:|
| data.table::yday(x)    |   4625.0 |   4788.2 |   208.5 | 7.63MB    |   33.9 |    86 |   14 |
| lubridate::yday(x)     | 236137.1 | 236137.1 |     4.2 | 64.85MB   |   16.9 |     1 |    4 |
| funchir::quick_yday(x) |  21883.4 |  22274.1 |    44.8 | 19.08MB   |   22.4 |    14 |    7 |
| ymd::yday(x)           |   6772.3 |   6954.2 |   143.3 | 3.82MB    |    4.1 |    70 |    2 |

``` r
run_bmk(
  data.table::mday(x),
  lubridate::mday(x),
  funchir::quick_mday(x),
  ymd::mday(x)
)
#> Warning: Some expressions had a GC in every iteration; so filtering is
#> disabled.
```

| expression             |      min |   median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:-----------------------|---------:|---------:|--------:|:----------|-------:|------:|-----:|
| data.table::mday(x)    |  19306.0 |  19851.4 |    49.9 | 7.63MB    |    4.0 |    25 |    2 |
| lubridate::mday(x)     | 201549.2 | 209073.6 |     4.5 | 57.22MB   |    4.5 |     3 |    3 |
| funchir::quick_mday(x) |   9063.7 |   9518.0 |    96.4 | 7.65MB    |    7.9 |    49 |    4 |
| ymd::mday(x)           |   7349.0 |   7622.6 |   130.0 | 3.82MB    |    6.0 |    65 |    3 |

``` r
run_bmk(
  data.table::wday(x),
  lubridate::wday(x),
  ymd::wday(x)
)
#> Warning: Some expressions had a GC in every iteration; so filtering is
#> disabled.
```

| expression          |      min |   median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:--------------------|---------:|---------:|--------:|:----------|-------:|------:|-----:|
| data.table::wday(x) |   1659.4 |   1750.8 |   506.9 | 7.63MB    |   61.9 |   254 |   31 |
| lubridate::wday(x)  | 207571.0 | 245077.7 |     4.3 | 64.85MB   |    8.6 |     3 |    6 |
| ymd::wday(x)        |   8603.3 |   8944.3 |   108.6 | 3.82MB    |    9.9 |    55 |    5 |

``` r
run_bmk(
  data.table::isoweek(x),
  lubridate::isoweek(x),
  ymd::isoweek(x)
)
#> Warning: Some expressions had a GC in every iteration; so filtering is
#> disabled.
```

| expression             |      min |   median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:-----------------------|---------:|---------:|--------:|:----------|-------:|------:|-----:|
| data.table::isoweek(x) | 552578.0 | 552578.0 |     1.8 | 122.08MB  |    3.6 |     1 |    2 |
| lubridate::isoweek(x)  | 524713.2 | 524713.2 |     1.9 | 278.5MB   |    9.5 |     1 |    5 |
| ymd::isoweek(x)        |   9559.8 |   9947.8 |    91.5 | 3.82MB    |    4.0 |    46 |    2 |

## Session Info

``` r
xfun::session_info()
#> R version 4.6.1 (2026-06-24)
#> Platform: aarch64-apple-darwin25.4.0
#> Running under: macOS Tahoe 26.5.2
#>
#> Locale: C.UTF-8 / C.UTF-8 / C.UTF-8 / C / C.UTF-8 / C.UTF-8
#>
#> Package version:
#>   base64enc_0.1.6   bench_1.1.4       bslib_0.12.0      cachem_1.1.0
#>   cli_3.6.6         clock_0.7.4       compiler_4.6.1    cpp11_0.5.5
#>   data.table_1.18.4 digest_0.6.39     evaluate_1.0.5    fastmap_1.2.0
#>   fontawesome_0.5.3 fs_2.1.0          funchir_0.3.0-1   generics_0.1.4
#>   glue_1.8.1        graphics_4.6.1    grDevices_4.6.1   highr_0.12
#>   htmltools_0.5.9   jquerylib_0.1.4   jsonlite_2.0.0    knitr_1.51
#>   lifecycle_1.0.5   lubridate_1.9.5   magrittr_2.0.5    memoise_2.0.1
#>   methods_4.6.1     mime_0.13         otel_0.2.0        pillar_1.11.1
#>   pkgconfig_2.0.3   profmem_0.7.0     R6_2.6.1          rappdirs_0.3.4
#>   rlang_1.3.0       rmarkdown_2.31    sass_0.4.10       stats_4.6.1
#>   tibble_3.3.1      timechange_0.4.0  tinytex_0.60      tools_4.6.1
#>   tzdb_0.5.0        utf8_1.2.6        utils_4.6.1       vctrs_0.7.3
#>   xfun_0.60         yaml_2.3.12       ymd_0.1.6
```
