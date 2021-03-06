
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

The binary package is provided by CRAN. So, if you are on Windows or
macOS, the package can be installed via:

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

In addition, you may download the binary package file generated by
GitHub Action from [the release
page](https://github.com/shrektan/ymd/releases) and install via:

``` r
install.packages("{the-downloaded-binary-pkg-file}", repos = NULL)
```

| artifact                                |     platform |
|:----------------------------------------|-------------:|
| ymd_0.0.1.tgz                           |  macOS Intel |
| ymd_0.0.1.zip                           |      Windows |
| ymd_0.0.1_R\_x86_64-pc-linux-gnu.tar.gz | Ubuntu-18.04 |

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
  x[, 1:8] |> lapply(rnd) |> as.data.frame() |> knitr::kable() |> print()
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
| ymd::ymd(x)       |   44.6 |   46.5 | 21102.2 | 228.31KB  |    0.0 | 10000 |    0 |
| lubridate::ymd(x) | 1572.5 | 1652.3 |   598.6 | 8.23MB    |   15.1 |   278 |    7 |

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
| ymd::ymd(x)       |   12.3 |   12.8 | 76949.0 | 3.17KB    |    0.0 | 10000 |    0 |
| lubridate::ymd(x) | 1743.2 | 1766.8 |   562.9 | 373.41KB  |   19.9 |   254 |    9 |

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
| ymd::ymd(x)       |  33.7 |   34.5 | 28859.7 | 2.39KB    |    0.0 | 10000 |    0 |
| lubridate::ymd(x) | 806.9 |  827.5 |  1196.6 | 201.1KB   |   19.5 |   552 |    9 |
| as.Date(x)        | 684.0 |  694.9 |  1438.5 | 87.54KB   |    0.0 |   720 |    0 |

``` r
x <- ymd::ymd(210515) + 1:100
run_bmk(
  ymd::eop$tm(x),
  lubridate::ceiling_date(x, "month") - 1
)
```

| expression                              |  min | median |  itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:----------------------------------------|-----:|-------:|---------:|:----------|-------:|------:|-----:|
| ymd::eop$tm(x)                          |  5.8 |    6.3 | 156615.1 | 19.3KB    |    0.0 | 10000 |    0 |
| lubridate::ceiling_date(x, “month”) - 1 | 97.7 |  102.5 |   9674.8 | 255.1KB   |   35.3 |  4388 |   16 |

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
| ymd::edate(x, 2) |  13.8 |   14.5 | 67718.5 | 6.2KB     |    6.8 |  9999 |    1 |
| x %m+% months(2) | 303.4 |  318.0 |  3141.9 | 424.6KB   |   23.5 |  1470 |   11 |

``` r
run_bmk(
  ymd::edate(x, -12),
  x %m+% months(-12)
)
```

| expression         |   min | median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:-------------------|------:|-------:|--------:|:----------|-------:|------:|-----:|
| ymd::edate(x, -12) |  13.9 |   14.6 | 67233.0 | 3.95KB    |    0.0 | 10000 |    0 |
| x %m+% months(-12) | 833.3 |  852.6 |  1166.6 | 317.19KB  |   36.2 |   516 |   16 |

### Extract Date Part

``` r
# tweak from https://github.com/Rdatatable/data.table/pull/5300
set.seed(373L)
x = as.Date(data.table::as.IDate(sample(seq(-25000, 45000), 1e6, TRUE)))

run_bmk(
  data.table::year(x),
  lubridate::year(x),
  funchir::quick_year(x),
  ymd::year(x)
)
```

| expression             |     min |  median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:-----------------------|--------:|--------:|--------:|:----------|-------:|------:|-----:|
| data.table::year(x)    | 87015.8 | 87015.8 |    11.5 | 41.97MB   |  114.9 |     1 |   10 |
| lubridate::year(x)     | 85443.0 | 85443.0 |    11.7 | 45.78MB   |   81.9 |     1 |    7 |
| funchir::quick_year(x) | 28843.3 | 29540.9 |    34.0 | 26.76MB   |   22.6 |     9 |    6 |
| ymd::year(x)           |  9431.0 | 10264.2 |    99.2 | 3.82MB    |    4.2 |    47 |    2 |

``` r
run_bmk(
  data.table::month(x),
  lubridate::month(x),
  ymd::month(x)
)
#> Warning: Some expressions had a GC in every iteration; so filtering is disabled.
```

| expression           |      min |   median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:---------------------|---------:|---------:|--------:|:----------|-------:|------:|-----:|
| data.table::month(x) |  88631.8 |  91172.8 |    10.5 | 41.97MB   |   14.0 |     6 |    8 |
| lubridate::month(x)  | 111172.1 | 112491.5 |     8.5 | 83.92MB   |   15.3 |     5 |    9 |
| ymd::month(x)        |   9786.2 |  10046.8 |    96.2 | 3.82MB    |    7.9 |    49 |    4 |

``` r
run_bmk(
  data.table::quarter(x),
  lubridate::quarter(x),
  ymd::quarter(x)
)
#> Warning: Some expressions had a GC in every iteration; so filtering is disabled.
```

| expression             |      min |   median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:-----------------------|---------:|---------:|--------:|:----------|-------:|------:|-----:|
| data.table::quarter(x) |  87279.2 |  88622.1 |    11.2 | 41.97MB   |    9.4 |     6 |    5 |
| lubridate::quarter(x)  | 123254.7 | 123753.1 |     7.7 | 99.21MB   |   15.3 |     4 |    8 |
| ymd::quarter(x)        |  16459.6 |  16735.2 |    58.5 | 3.82MB    |    2.0 |    30 |    1 |

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
| data.table::yday(x)    | 86179.1 | 86230.8 |    11.6 | 41.97MB   |   29.0 |     2 |    5 |
| lubridate::yday(x)     | 83891.9 | 83891.9 |    11.9 | 45.78MB   |  107.3 |     1 |    9 |
| funchir::quick_yday(x) | 22771.1 | 23299.0 |    43.1 | 19.08MB   |   21.5 |    14 |    7 |
| ymd::yday(x)           |  9520.9 | 10247.7 |    98.0 | 3.82MB    |    6.4 |    46 |    3 |

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
| data.table::mday(x)    | 83080.3 | 83509.9 |    12.0 | 38.15MB   |   12.0 |     3 |    3 |
| lubridate::mday(x)     | 83465.8 | 83465.8 |    12.0 | 38.15MB   |   59.9 |     1 |    5 |
| funchir::quick_mday(x) |  8658.7 |  9155.3 |   109.3 | 15.28MB   |   25.9 |    38 |    9 |
| ymd::mday(x)           |  9791.4 | 10025.7 |    99.6 | 3.82MB    |    6.4 |    47 |    3 |

``` r
run_bmk(
  data.table::wday(x),
  lubridate::wday(x),
  ymd::wday(x)
)
```

| expression          |     min |  median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:--------------------|--------:|--------:|--------:|:----------|-------:|------:|-----:|
| data.table::wday(x) | 10430.6 | 10592.4 |    94.4 | 3.82MB    |    4.2 |    45 |    2 |
| lubridate::wday(x)  | 84564.4 | 84606.7 |    11.8 | 45.78MB   |   11.8 |     3 |    3 |
| ymd::wday(x)        | 10266.3 | 10483.2 |    95.8 | 3.82MB    |    6.4 |    45 |    3 |

``` r
run_bmk(
  data.table::isoweek(x),
  lubridate::isoweek(x),
  ymd::isoweek(x)
)
#> Warning: Some expressions had a GC in every iteration; so filtering is disabled.
```

| expression             |       min |    median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:-----------------------|----------:|----------:|--------:|:----------|-------:|------:|-----:|
| data.table::isoweek(x) | 2839905.2 | 2839905.2 |     0.4 | 236.59MB  |    1.8 |     1 |    5 |
| lubridate::isoweek(x)  |  262504.2 |  267657.6 |     3.7 | 247.97MB  |   18.7 |     2 |   10 |
| ymd::isoweek(x)        |   11244.5 |   11414.3 |    85.1 | 3.82MB    |    2.0 |    43 |    1 |

## Session Info

``` r
xfun::session_info()
#> R version 4.1.2 (2021-11-01)
#> Platform: aarch64-apple-darwin20 (64-bit)
#> Running under: macOS Monterey 12.2.1
#> 
#> Locale: en_US.UTF-8 / en_US.UTF-8 / en_US.UTF-8 / C / en_US.UTF-8 / en_US.UTF-8
#> 
#> Package version:
#>   base64enc_0.1.3   bench_1.1.2       cli_3.1.0         compiler_4.1.2   
#>   cpp11_0.4.2       crayon_1.4.2      data.table_1.14.3 digest_0.6.29    
#>   ellipsis_0.3.2    evaluate_0.14     fansi_0.5.0       fastmap_1.1.0    
#>   funchir_0.2.0     generics_0.1.1    glue_1.6.0        graphics_4.1.2   
#>   grDevices_4.1.2   highr_0.9         htmltools_0.5.2   jquerylib_0.1.4  
#>   jsonlite_1.7.2    knitr_1.37.7      lifecycle_1.0.1   lubridate_1.8.0  
#>   magrittr_2.0.1    methods_4.1.2     pillar_1.6.4      pkgconfig_2.0.3  
#>   profmem_0.6.0     rlang_0.4.12      rmarkdown_2.11    stats_4.1.2      
#>   stringi_1.7.6     stringr_1.4.0     tibble_3.1.6      tinytex_0.35     
#>   tools_4.1.2       utf8_1.2.2        utils_4.1.2       vctrs_0.3.8      
#>   xfun_0.30         yaml_2.2.1        ymd_0.0.5
```
