
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
| ymd::ymd(x)       |   32.0 |   32.8 | 30037.1 | 812.97KB  |    0.0 | 10000 |    0 |
| lubridate::ymd(x) | 1376.4 | 1457.9 |   671.3 | 8.71MB    |   13.2 |   306 |    6 |

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
| ymd::ymd(x)       |   13.1 |   13.7 | 70210.9 | 3.17KB    |    0.0 | 10000 |    0 |
| lubridate::ymd(x) | 1705.4 | 1785.1 |   556.1 | 362.21KB  |   17.5 |   254 |    8 |

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
| ymd::ymd(x)       |  24.4 |   25.9 | 37791.9 | 2.39KB    |    3.8 |  9999 |    1 |
| lubridate::ymd(x) | 791.8 |  822.2 |  1200.3 | 193.52KB  |   17.3 |   554 |    8 |
| as.Date(x)        | 641.1 |  672.4 |  1488.4 | 87.54KB   |    0.0 |   745 |    0 |

``` r

x <- ymd::ymd(210515) + 1:100
run_bmk(
  ymd::eop$tm(x),
  lubridate::ceiling_date(x, "month") - 1
)
```

| expression                              |  min | median |  itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:----------------------------------------|-----:|-------:|---------:|:----------|-------:|------:|-----:|
| ymd::eop\$tm(x)                         |  5.9 |    6.6 | 149381.8 | 19.3KB    |      0 | 10000 |    0 |
| lubridate::ceiling_date(x, “month”) - 1 | 32.8 |   34.7 |  28141.0 | 155.4KB   |     31 |  9989 |   11 |

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
| ymd::edate(x, 2) |  14.1 |   14.9 | 66499.9 | 6.2KB     |    0.0 | 10000 |    0 |
| x %m+% months(2) | 311.9 |  324.5 |  3029.7 | 459.8KB   |   23.6 |  1412 |   11 |

``` r
run_bmk(
  ymd::edate(x, -12),
  x %m+% months(-12)
)
```

| expression         |   min | median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:-------------------|------:|-------:|--------:|:----------|-------:|------:|-----:|
| ymd::edate(x, -12) |  14.2 |   15.0 | 66097.6 | 3.95KB    |    0.0 | 10000 |    0 |
| x %m+% months(-12) | 647.6 |  666.2 |  1471.8 | 286.83KB  |   28.1 |   682 |   13 |

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
| data.table::year(x)    | 84590.6 | 85742.5 |    10.6 | 41.97MB   |   14.1 |     6 |    8 |
| lubridate::year(x)     | 84620.6 | 86198.4 |    11.1 | 45.78MB   |   16.6 |     6 |    9 |
| funchir::quick_year(x) | 28513.7 | 29036.1 |    30.2 | 26.76MB   |   11.3 |    16 |    6 |
| ymd::year(x)           |  8005.8 |  8410.3 |   115.3 | 3.82MB    |    8.0 |    58 |    4 |

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
| data.table::month(x) |  84417.1 |  86762.9 |    11.5 | 41.97MB   |    7.7 |     6 |    4 |
| lubridate::month(x)  | 106067.5 | 107216.5 |     8.9 | 83.92MB   |   21.3 |     5 |   12 |
| ymd::month(x)        |   9007.5 |   9828.8 |   101.6 | 3.82MB    |    8.0 |    51 |    4 |

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
| data.table::quarter(x) |  87365.4 |  93648.3 |    10.3 | 41.97MB   |    8.6 |     6 |    5 |
| lubridate::quarter(x)  | 124762.3 | 129507.2 |     7.6 | 99.21MB   |   11.4 |     4 |    6 |
| ymd::quarter(x)        |  16361.0 |  17152.9 |    57.5 | 3.82MB    |    4.0 |    29 |    2 |

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
| data.table::yday(x)    | 84184.1 | 84490.6 |    11.8 | 41.97MB   |   41.4 |     2 |    7 |
| lubridate::yday(x)     | 89058.9 | 89058.9 |    11.2 | 45.78MB   |   89.8 |     1 |    8 |
| funchir::quick_yday(x) | 22964.3 | 23207.3 |    42.9 | 19.08MB   |   39.0 |    11 |   10 |
| ymd::yday(x)           |  9580.8 |  9902.8 |    99.6 | 3.82MB    |    8.9 |    45 |    4 |

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
| data.table::mday(x)    | 81525.0 | 82372.6 |    12.2 | 38.15MB   |   12.2 |     3 |    3 |
| lubridate::mday(x)     | 81730.8 | 81730.8 |    12.2 | 38.15MB   |   61.2 |     1 |    5 |
| funchir::quick_mday(x) |  9127.1 |  9380.6 |   106.4 | 15.28MB   |   21.3 |    35 |    7 |
| ymd::mday(x)           |  9145.7 | 10157.8 |   100.0 | 3.82MB    |    4.2 |    48 |    2 |

``` r
run_bmk(
  data.table::wday(x),
  lubridate::wday(x),
  ymd::wday(x)
)
```

| expression          |     min |  median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:--------------------|--------:|--------:|--------:|:----------|-------:|------:|-----:|
| data.table::wday(x) | 10252.4 | 10449.1 |    95.5 | 3.82MB    |    4.2 |    45 |    2 |
| lubridate::wday(x)  | 83525.2 | 84106.4 |    11.9 | 45.78MB   |   29.7 |     2 |    5 |
| ymd::wday(x)        |  9127.3 | 10192.6 |    98.8 | 3.82MB    |    9.0 |    44 |    4 |

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
| data.table::isoweek(x) | 2737848.3 | 2737848.3 |     0.4 | 225.14MB  |    1.8 |     1 |    5 |
| lubridate::isoweek(x)  |  265307.8 |  269441.4 |     3.7 | 248MB     |   18.6 |     2 |   10 |
| ymd::isoweek(x)        |   10436.8 |   10700.4 |    90.2 | 3.82MB    |    3.9 |    46 |    2 |

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
