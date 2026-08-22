# ymd

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

remotes::install_github("shrektan/ymd")
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
| ymd::ymd(x)       |   41.8 |   43.3 | 21861.1 | 810.52KB  |    2.2 |  9999 |    1 |
| lubridate::ymd(x) | 1365.3 | 1534.8 |   627.8 | 9.11MB    |   13.2 |   286 |    6 |

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
| ymd::ymd(x)       |    4.1 |    4.8 | 179995.6 | 3.17KB    |     18 |  9999 |    1 |
| lubridate::ymd(x) | 1688.7 | 1864.3 |    508.7 | 365.38KB  |     15 |   237 |    7 |

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
| ymd::ymd(x) | 30.5 | 32.0 | 30464.0 | 2.39KB | 0.0 | 10000 | 0 |
| clock::as_date(clock::year_month_day_parse(x)) | 160.6 | 177.9 | 3698.0 | 3.66MB | 14.9 | 1734 | 7 |
| lubridate::ymd(x) | 792.3 | 930.1 | 940.8 | 193.52KB | 12.9 | 436 | 6 |
| as.Date(x) | 1054.4 | 1090.4 | 889.7 | 101.69KB | 2.0 | 441 | 1 |

``` r


x <- ymd::ymd(210515) + 1:100
run_bmk(
  ymd::eop$tm(x),
  clock::as_date(clock::calendar_end(clock::as_year_month_day(x), "month")),
  lubridate::ceiling_date(x, "month") - 1
)
```

| expression | min | median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:---|---:|---:|---:|:---|---:|---:|---:|
| ymd::eop\$tm(x) | 5.8 | 6.4 | 148901.5 | 19.3KB | 0.0 | 10000 | 0 |
| clock::as_date(clock::calendar_end(clock::as_year_month_day(x), “month”)) | 108.9 | 115.8 | 7867.6 | 34.9KB | 30.1 | 3664 | 14 |
| lubridate::ceiling_date(x, “month”) - 1 | 35.4 | 38.0 | 24469.2 | 159.5KB | 29.4 | 9988 | 12 |

### edate

``` r

`%m+%` <- lubridate::`%m+%`
x <- ymd::ymd(c(200115, 200131, 200229, 200331, 200401))
x <- rep(x, 100)
run_bmk(
  ymd::edate(x, 2),
  clock::add_months(x, 2, invalid = "previous"),
  x %m+% months(2)
)
```

| expression | min | median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:---|---:|---:|---:|:---|---:|---:|---:|
| ymd::edate(x, 2) | 12.3 | 13.3 | 73041.1 | 6.2KB | 0.0 | 10000 | 0 |
| clock::add_months(x, 2, invalid = “previous”) | 157.2 | 166.9 | 5423.6 | 161KB | 25.7 | 2534 | 12 |
| x %m+% months(2) | 1354.3 | 1431.7 | 675.3 | 496.5KB | 6.1 | 332 | 3 |

``` r

run_bmk(
  ymd::edate(x, -12),
  clock::add_months(x, -12, invalid = "previous"),
  x %m+% months(-12)
)
```

| expression | min | median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:---|---:|---:|---:|:---|---:|---:|---:|
| ymd::edate(x, -12) | 12.3 | 13.2 | 72121.2 | 3.95KB | 7.2 | 9999 | 1 |
| clock::add_months(x, -12, invalid = “previous”) | 158.5 | 169.1 | 5191.7 | 65.65KB | 23.3 | 2455 | 11 |
| x %m+% months(-12) | 1751.0 | 1986.7 | 487.9 | 310.65KB | 9.1 | 215 | 4 |

### Extract Date Part

``` r

# tweak from https://github.com/Rdatatable/data.table/pull/5300
set.seed(373L)
x <- as.Date(data.table::as.IDate(sample(seq(-25000, 45000), 1e6, TRUE)))

run_bmk(
  data.table::year(x),
  clock::get_year(x),
  lubridate::year(x),
  funchir::quick_year(x),
  ymd::year(x)
)
#> Warning: Some expressions had a GC in every iteration; so filtering is
#> disabled.
```

| expression             |      min |   median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:-----------------------|---------:|---------:|--------:|:----------|-------:|------:|-----:|
| data.table::year(x)    |   3353.1 |   3502.2 |   260.6 | 7.64MB    |   55.7 |   131 |   28 |
| clock::get_year(x)     |  17215.0 |  18977.2 |    41.8 | 30.53MB   |   39.8 |    21 |   20 |
| lubridate::year(x)     | 205532.1 | 238408.2 |     4.3 | 64.85MB   |    7.2 |     3 |    5 |
| funchir::quick_year(x) |  25727.2 |  27358.1 |    33.8 | 22.93MB   |    9.9 |    17 |    5 |
| ymd::year(x)           |   6806.8 |   6942.4 |   140.5 | 3.82MB    |    9.9 |    71 |    5 |

``` r

run_bmk(
  data.table::month(x),
  clock::get_month(x),
  lubridate::month(x),
  ymd::month(x)
)
#> Warning: Some expressions had a GC in every iteration; so filtering is
#> disabled.
```

| expression           |      min |   median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:---------------------|---------:|---------:|--------:|:----------|-------:|------:|-----:|
| data.table::month(x) |  19978.4 |  20979.9 |    47.7 | 7.63MB    |    7.9 |    24 |    4 |
| clock::get_month(x)  |  17698.4 |  18647.8 |    48.9 | 26.71MB   |   27.4 |    25 |   14 |
| lubridate::month(x)  | 225250.3 | 254814.8 |     3.9 | 103MB     |    5.9 |     2 |    3 |
| ymd::month(x)        |   7264.4 |   7476.1 |   131.6 | 3.82MB    |   10.0 |    66 |    5 |

``` r

run_bmk(
  data.table::quarter(x),
  clock::get_quarter(clock::as_year_quarter_day(x)),
  lubridate::quarter(x),
  ymd::quarter(x)
)
#> Warning: Some expressions had a GC in every iteration; so filtering is
#> disabled.
```

| expression | min | median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:---|---:|---:|---:|:---|---:|---:|---:|
| data.table::quarter(x) | 16695.6 | 17065.9 | 57.6 | 7.63MB | 6.0 | 29 | 3 |
| clock::get_quarter(clock::as_year_quarter_day(x)) | 22743.8 | 24452.2 | 37.6 | 26.73MB | 19.8 | 19 | 10 |
| lubridate::quarter(x) | 244914.8 | 250065.3 | 4.0 | 118.29MB | 8.0 | 2 | 4 |
| ymd::quarter(x) | 13689.2 | 14148.4 | 69.3 | 3.82MB | 5.9 | 35 | 3 |

``` r

run_bmk(
  data.table::yday(x),
  clock::get_day(clock::as_year_day(x)),
  lubridate::yday(x),
  funchir::quick_yday(x),
  ymd::yday(x)
)
#> Warning: Some expressions had a GC in every iteration; so filtering is
#> disabled.
```

| expression | min | median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:---|---:|---:|---:|:---|---:|---:|---:|
| data.table::yday(x) | 4639.7 | 4864.0 | 199.9 | 7.63MB | 13.9 | 101 | 7 |
| clock::get_day(clock::as_year_day(x)) | 19214.7 | 21036.1 | 43.8 | 22.9MB | 12.0 | 22 | 6 |
| lubridate::yday(x) | 212040.0 | 214421.6 | 4.6 | 64.85MB | 6.2 | 3 | 4 |
| funchir::quick_yday(x) | 22006.7 | 23076.6 | 39.8 | 19.08MB | 13.9 | 20 | 7 |
| ymd::yday(x) | 6828.4 | 7047.6 | 140.4 | 3.82MB | 4.0 | 71 | 2 |

``` r

run_bmk(
  data.table::mday(x),
  clock::get_day(x),
  lubridate::mday(x),
  funchir::quick_mday(x),
  ymd::mday(x)
)
#> Warning: Some expressions had a GC in every iteration; so filtering is
#> disabled.
```

| expression             |      min |   median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:-----------------------|---------:|---------:|--------:|:----------|-------:|------:|-----:|
| data.table::mday(x)    |  19289.1 |  19788.3 |    49.2 | 7.63MB    |    2.0 |    25 |    1 |
| clock::get_day(x)      |  17082.0 |  17939.6 |    54.7 | 26.71MB   |   19.5 |    28 |   10 |
| lubridate::mday(x)     | 207501.9 | 222374.2 |     4.4 | 57.22MB   |    4.4 |     3 |    3 |
| funchir::quick_mday(x) |   9333.4 |  10908.9 |    81.9 | 7.65MB    |    8.0 |    41 |    4 |
| ymd::mday(x)           |   7366.5 |   9651.2 |    99.6 | 3.82MB    |    2.0 |    51 |    1 |

``` r

run_bmk(
  data.table::wday(x),
  clock::weekday_code(clock::as_weekday(x)),
  lubridate::wday(x),
  ymd::wday(x)
)
```

| expression | min | median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:---|---:|---:|---:|:---|---:|---:|---:|
| data.table::wday(x) | 1671.6 | 2153.2 | 363.4 | 7.63MB | 30.9 | 153 | 13 |
| clock::weekday_code(clock::as_weekday(x)) | 10204.5 | 13697.9 | 69.3 | 22.91MB | 24.9 | 25 | 9 |
| lubridate::wday(x) | 258137.3 | 258137.3 | 3.9 | 64.85MB | 3.9 | 1 | 1 |
| ymd::wday(x) | 9279.6 | 10908.8 | 89.6 | 3.82MB | 6.6 | 41 | 3 |

``` r

run_bmk(
  data.table::isoweek(x),
  clock::get_week(clock::as_iso_year_week_day(x)),
  lubridate::isoweek(x),
  ymd::isoweek(x)
)
#> Warning: Some expressions had a GC in every iteration; so filtering is
#> disabled.
```

| expression | min | median | itr.sec | mem_alloc | gc.sec | n_itr | n_gc |
|:---|---:|---:|---:|:---|---:|---:|---:|
| data.table::isoweek(x) | 601227.6 | 601227.6 | 1.7 | 122.08MB | 6.7 | 1 | 4 |
| clock::get_week(clock::as_iso_year_week_day(x)) | 31462.4 | 38713.8 | 23.4 | 26.72MB | 9.7 | 12 | 5 |
| lubridate::isoweek(x) | 625706.3 | 625706.3 | 1.6 | 278.5MB | 9.6 | 1 | 6 |
| ymd::isoweek(x) | 9738.4 | 10692.3 | 90.7 | 3.82MB | 3.9 | 46 | 2 |

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
