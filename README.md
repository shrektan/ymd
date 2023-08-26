
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
| ymd::ymd(x)       |   32.1 |   33.0 | 29177.6 | 812.97KB  |    0.0 | 10000 |    0 |
| lubridate::ymd(x) | 1380.9 | 1439.9 |   687.4 | 8.71MB    |   13.1 |   316 |    6 |

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
| ymd::ymd(x)       |   13.1 |   14.1 | 68876.6 | 3.17KB    |    0.0 | 10000 |    0 |
| lubridate::ymd(x) | 1723.1 | 1843.1 |   532.1 | 362.21KB  |   17.6 |   242 |    8 |

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
| ymd::ymd(x)       |  24.4 |   25.7 | 38681.7 | 2.39KB    |    0.0 | 10000 |    0 |
| lubridate::ymd(x) | 794.3 |  825.0 |  1195.4 | 193.52KB  |   19.6 |   548 |    9 |
| as.Date(x)        | 641.5 |  651.0 |  1521.5 | 87.54KB   |    0.0 |   761 |    0 |

``` r

x <- ymd::ymd(210515) + 1:100
run_bmk(
  ymd::eop$tm(x),
  lubridate::ceiling_date(x, "month") - 1
)
```

| expression                              |  min | median |  itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:----------------------------------------|-----:|-------:|---------:|:----------|-------:|------:|-----:|
| ymd::eop\$tm(x)                         |  6.2 |    6.8 | 144267.2 | 19.3KB    |   14.4 |  9999 |    1 |
| lubridate::ceiling_date(x, “month”) - 1 | 33.0 |   35.9 |  26277.9 | 155.4KB   |   26.3 |  9990 |   10 |

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

| expression       |   min | median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:-----------------|------:|-------:|--------:|:----------|-------:|------:|-----:|
| ymd::edate(x, 2) |  14.2 |   15.3 | 60822.1 | 6.2KB     |    0.0 | 10000 |    0 |
| x %m+% months(2) | 312.9 |  324.4 |  3031.8 | 459.8KB   |   23.5 |  1417 |   11 |

``` r
run_bmk(
  ymd::edate(x, -12),
  x %m+% months(-12)
)
```

| expression         |   min | median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:-------------------|------:|-------:|--------:|:----------|-------:|------:|-----:|
| ymd::edate(x, -12) |  14.3 |   15.1 | 65395.1 | 3.95KB    |    0.0 | 10000 |    0 |
| x %m+% months(-12) | 649.9 |  693.0 |  1410.8 | 286.83KB  |   28.3 |   649 |   13 |

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

| expression             |     min |  median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:-----------------------|--------:|--------:|--------:|:----------|-------:|------:|-----:|
| data.table::year(x)    | 86831.9 | 89393.4 |    10.0 | 41.97MB   |   12.0 |     5 |    6 |
| lubridate::year(x)     | 86283.8 | 88587.6 |    10.8 | 45.78MB   |   16.1 |     6 |    9 |
| funchir::quick_year(x) | 28862.1 | 29720.6 |    29.0 | 26.76MB   |   11.6 |    15 |    6 |
| ymd::year(x)           |  8113.3 |  9021.5 |   110.4 | 3.82MB    |    5.9 |    56 |    3 |

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
| data.table::month(x) |  84331.8 |  85835.8 |    10.9 | 41.97MB   |    9.1 |     6 |    5 |
| lubridate::month(x)  | 106749.5 | 109019.0 |     8.7 | 83.92MB   |   15.7 |     5 |    9 |
| ymd::month(x)        |   9231.3 |  10170.6 |    96.5 | 3.82MB    |    5.9 |    49 |    3 |

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
| data.table::quarter(x) |  85807.7 |  89130.3 |    10.7 | 41.97MB   |    8.9 |     6 |    5 |
| lubridate::quarter(x)  | 119922.3 | 122357.1 |     7.6 | 99.21MB   |   15.3 |     4 |    8 |
| ymd::quarter(x)        |  16333.4 |  16719.0 |    59.0 | 3.82MB    |    3.9 |    30 |    2 |

``` r
run_bmk(
  data.table::yday(x),
  lubridate::yday(x),
  funchir::quick_yday(x),
  ymd::yday(x)
)
```

| expression             |     min |  median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:-----------------------|--------:|--------:|--------:|:----------|-------:|------:|-----:|
| data.table::yday(x)    | 84639.5 | 86600.7 |    11.6 | 41.97MB   |   15.5 |     3 |    4 |
| lubridate::yday(x)     | 86884.6 | 86884.6 |    11.5 | 45.78MB   |   92.1 |     1 |    8 |
| funchir::quick_yday(x) | 22892.4 | 22943.2 |    43.6 | 19.08MB   |   39.6 |    11 |   10 |
| ymd::yday(x)           |  8831.9 |  9686.3 |   104.3 | 3.82MB    |   11.3 |    46 |    5 |

``` r
run_bmk(
  data.table::mday(x),
  lubridate::mday(x),
  funchir::quick_mday(x),
  ymd::mday(x)
)
```

| expression             |     min |  median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:-----------------------|--------:|--------:|--------:|:----------|-------:|------:|-----:|
| data.table::mday(x)    | 83673.9 | 83947.1 |    11.9 | 38.15MB   |   23.8 |     2 |    4 |
| lubridate::mday(x)     | 83454.8 | 83782.7 |    11.9 | 38.15MB   |   23.9 |     2 |    4 |
| funchir::quick_mday(x) |  8906.7 |  9319.6 |   104.9 | 15.28MB   |   21.0 |    35 |    7 |
| ymd::mday(x)           | 10199.6 | 10472.3 |    95.1 | 3.82MB    |    4.2 |    45 |    2 |

``` r
run_bmk(
  data.table::wday(x),
  lubridate::wday(x),
  ymd::wday(x)
)
```

| expression          |     min |  median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:--------------------|--------:|--------:|--------:|:----------|-------:|------:|-----:|
| data.table::wday(x) | 10256.9 | 10561.3 |    94.9 | 3.82MB    |    8.8 |    43 |    4 |
| lubridate::wday(x)  | 83052.8 | 83843.6 |    11.9 | 45.78MB   |   35.8 |     2 |    6 |
| ymd::wday(x)        |  8949.8 |  9399.4 |   105.3 | 3.82MB    |   13.7 |    46 |    6 |

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
| data.table::isoweek(x) | 2788300.2 | 2788300.2 |     0.4 | 225.14MB  |    1.8 |     1 |    5 |
| lubridate::isoweek(x)  |  244948.8 |  276808.1 |     3.6 | 248MB     |   10.8 |     2 |    6 |
| ymd::isoweek(x)        |   11160.7 |   11834.5 |    82.2 | 3.82MB    |    5.9 |    42 |    3 |

## Session Info

``` r
xfun::session_info()
#> R version 4.2.1 (2022-06-23)
#> Platform: aarch64-apple-darwin20 (64-bit)
#> Running under: macOS Ventura 13.4.1
#> 
#> Locale: en_US.UTF-8 / en_US.UTF-8 / en_US.UTF-8 / C / en_US.UTF-8 / en_US.UTF-8
#> 
#> Package version:
#>   base64enc_0.1.3   bench_1.1.3       bslib_0.5.1       cachem_1.0.8     
#>   cli_3.6.1         compiler_4.2.1    cpp11_0.4.6       data.table_1.14.8
#>   digest_0.6.33     ellipsis_0.3.2    evaluate_0.21     fansi_1.0.4      
#>   fastmap_1.1.1     fontawesome_0.5.2 fs_1.6.3          funchir_0.2.2    
#>   generics_0.1.3    glue_1.6.2        graphics_4.2.1    grDevices_4.2.1  
#>   highr_0.10        htmltools_0.5.6   jquerylib_0.1.4   jsonlite_1.8.7   
#>   knitr_1.43        lifecycle_1.0.3   lubridate_1.9.2   magrittr_2.0.3   
#>   memoise_2.0.1     methods_4.2.1     mime_0.12         pillar_1.9.0     
#>   pkgconfig_2.0.3   profmem_0.6.0     R6_2.5.1          rappdirs_0.3.3   
#>   rlang_1.1.1       rmarkdown_2.24    sass_0.4.7        stats_4.2.1      
#>   stringi_1.7.12    stringr_1.5.0     tibble_3.2.1      timechange_0.2.0 
#>   tinytex_0.46      tools_4.2.1       utf8_1.2.3        utils_4.2.1      
#>   vctrs_0.6.3       xfun_0.40         yaml_2.3.7        ymd_0.1.0
```
