
<!-- README.md is generated  from README.Rmd. Please edit that file -->

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
| ymd::ymd(x)       |   37.8 |   39.5 | 24224.5 | 810.1KB   |      0 | 10000 |    0 |
| lubridate::ymd(x) | 1414.8 | 1591.1 |   587.8 | 8.98MB    |     11 |   268 |    5 |

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
| ymd::ymd(x)       |    4.1 |    4.7 | 192504.8 | 3.17KB    |      0 | 10000 |    0 |
| lubridate::ymd(x) | 1761.8 | 1925.6 |    492.8 | 365.38KB  |     13 |   228 |    6 |

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
| ymd::ymd(x)       |  27.5 |   29.1 | 29997.8 | 2.39KB    |    0.0 | 10000 |    0 |
| lubridate::ymd(x) | 808.8 |  899.9 |   952.2 | 193.52KB  |   12.8 |   445 |    6 |
| as.Date(x)        | 832.3 |  880.8 |  1100.0 | 85.44KB   |    0.0 |   550 |    0 |

``` r
x <- ymd::ymd(210515) + 1:100
run_bmk(
  ymd::eop$tm(x),
  lubridate::ceiling_date(x, "month") - 1
)
```

| expression                              |  min | median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:----------------------------------------|-----:|-------:|--------:|:----------|-------:|------:|-----:|
| ymd::eop\$tm(x)                         |  5.8 |    6.5 | 95222.9 | 19.3KB    |    0.0 | 10000 |    0 |
| lubridate::ceiling_date(x, “month”) - 1 | 34.3 |   38.0 | 23779.2 | 155.5KB   |   23.8 |  9990 |   10 |

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
| ymd::edate(x, 2) |   12.8 |   13.9 | 69638.9 | 6.2KB     |    0.0 | 10000 |    0 |
| x %m+% months(2) | 1182.5 | 1259.8 |   767.4 | 496.8KB   |    6.2 |   374 |    3 |

``` r
run_bmk(
  ymd::edate(x, -12),
  x %m+% months(-12)
)
```

| expression         |    min | median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:-------------------|-------:|-------:|--------:|:----------|-------:|------:|-----:|
| ymd::edate(x, -12) |   12.9 |   13.6 | 69390.1 | 3.95KB    |    6.9 |  9999 |    1 |
| x %m+% months(-12) | 1519.1 | 1598.7 |   603.7 | 310.64KB  |   10.4 |   290 |    5 |

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
| data.table::year(x)    |   4585.9 |   5416.8 |   152.4 | 7.64MB    |   37.6 |    77 |   19 |
| lubridate::year(x)     | 294390.5 | 299745.7 |     3.3 | 57.23MB   |    6.7 |     2 |    4 |
| funchir::quick_year(x) |  31250.7 |  31840.8 |    26.4 | 26.76MB   |    7.5 |    14 |    4 |
| ymd::year(x)           |   8761.9 |   9779.2 |   100.2 | 3.82MB    |    3.9 |    51 |    2 |

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
| data.table::month(x) |  22814.2 |  24120.2 |    36.0 | 7.63MB    |    6.0 |    18 |    3 |
| lubridate::month(x)  | 291692.5 | 330627.5 |     3.0 | 95.37MB   |    4.5 |     2 |    3 |
| ymd::month(x)        |   8999.5 |   9907.7 |    97.6 | 3.82MB    |    8.0 |    49 |    4 |

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
| data.table::quarter(x) |  18667.2 |  19201.8 |    51.4 | 7.63MB    |    5.9 |    26 |    3 |
| lubridate::quarter(x)  | 295815.7 | 311213.9 |     3.2 | 110.66MB  |    8.0 |     2 |    5 |
| ymd::quarter(x)        |  16038.6 |  16756.7 |    54.4 | 3.82MB    |    1.9 |    28 |    1 |

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
| data.table::yday(x)    |   8636.1 |   9384.6 |   101.1 | 7.63MB    |   17.8 |    51 |    9 |
| lubridate::yday(x)     | 256363.1 | 259132.9 |     3.9 | 57.23MB   |    5.8 |     2 |    3 |
| funchir::quick_yday(x) |  24603.2 |  25937.3 |    37.8 | 19.08MB   |   17.9 |    19 |    9 |
| ymd::yday(x)           |   8815.9 |   9605.1 |    93.9 | 3.82MB    |    4.0 |    47 |    2 |

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
| data.table::mday(x)    |  21353.3 |  22150.6 |    44.5 | 7.63MB    |    7.7 |    23 |    4 |
| lubridate::mday(x)     | 255903.8 | 272992.9 |     3.7 | 49.59MB   |    5.5 |     2 |    3 |
| funchir::quick_mday(x) |   9126.2 |   9729.8 |    87.0 | 15.28MB   |   11.9 |    44 |    6 |
| ymd::mday(x)           |   8909.1 |   9751.1 |   100.7 | 3.82MB    |    5.9 |    51 |    3 |

``` r
run_bmk(
  data.table::wday(x),
  lubridate::wday(x),
  ymd::wday(x)
)
```

| expression          |      min |   median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:--------------------|---------:|---------:|--------:|:----------|-------:|------:|-----:|
| data.table::wday(x) |   2711.5 |   3095.9 |   322.2 | 7.63MB    |   28.6 |   135 |   12 |
| lubridate::wday(x)  | 251628.6 | 251628.6 |     4.0 | 57.22MB   |    4.0 |     1 |    1 |
| ymd::wday(x)        |  10693.7 |  11469.9 |    87.1 | 3.82MB    |    6.5 |    40 |    3 |

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
| data.table::isoweek(x) | 3774463.7 | 3774463.7 |     0.3 | 259.48MB  |    1.6 |     1 |    6 |
| lubridate::isoweek(x)  |  628920.6 |  628920.6 |     1.6 | 270.87MB  |    6.4 |     1 |    4 |
| ymd::isoweek(x)        |   11175.6 |   12156.0 |    75.9 | 3.82MB    |    2.0 |    38 |    1 |

## Session Info

``` r
xfun::session_info()
#> R version 4.4.2 (2024-10-31)
#> Platform: aarch64-apple-darwin20
#> Running under: macOS Sequoia 15.4
#> 
#> Locale: en_US.UTF-8 / en_US.UTF-8 / en_US.UTF-8 / C / en_US.UTF-8 / en_US.UTF-8
#> 
#> Package version:
#>   base64enc_0.1.3   bench_1.1.3       bslib_0.8.0       cachem_1.1.0     
#>   cli_3.6.3         compiler_4.4.2    cpp11_0.4.7       data.table_1.16.2
#>   digest_0.6.36     evaluate_0.24.0   fansi_1.0.6       fastmap_1.2.0    
#>   fontawesome_0.5.2 fs_1.6.4          funchir_0.2.2     generics_0.1.3   
#>   glue_1.7.0        graphics_4.4.2    grDevices_4.4.2   highr_0.11       
#>   htmltools_0.5.8.1 jquerylib_0.1.4   jsonlite_1.8.8    knitr_1.48       
#>   lifecycle_1.0.4   lubridate_1.9.3   magrittr_2.0.3    memoise_2.0.1    
#>   methods_4.4.2     mime_0.12         pillar_1.9.0      pkgconfig_2.0.3  
#>   profmem_0.6.0     R6_2.5.1          rappdirs_0.3.3    rlang_1.1.4      
#>   rmarkdown_2.27    sass_0.4.9        stats_4.4.2       tibble_3.2.1     
#>   timechange_0.3.0  tinytex_0.52      tools_4.4.2       utf8_1.2.4       
#>   utils_4.4.2       vctrs_0.6.5       xfun_0.50.6       yaml_2.3.10      
#>   ymd_0.1.5
```
