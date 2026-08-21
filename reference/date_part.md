# Fast Date Part Extracting

These date helper functions provide the similar functionalities like in
`data.table` or `lubridate` package. They are implemented by the Rust
Lang's standard library and very fast.

## Usage

``` r
year(ref_date)

month(ref_date)

quarter(ref_date)

isoweek(ref_date)

isowday(ref_date)

wday(ref_date)

mday(ref_date)

yday(ref_date)
```

## Arguments

- ref_date, :

  a Date vector. It will try to convert the input to date via
  [`ymd()`](https://shrektan.github.io/ymd/reference/ymd.md), if the
  input is not a Date.

## Value

an integer vector

## Details

- year, month, quarter: get the year, month, quarter part

- yday: the day of year

- mday: the day of month

- wday: the day of the week (Sunday is 1)

- isoweek: ISO 8601 week

- isowday: the day of week (ISO 8601 weekday number, Monday is 1)

## References

ISO week day, https://en.wikipedia.org/wiki/ISO_week_date ISO 8601,
https://en.wikipedia.org/wiki/ISO_8601

## Examples

``` r
year(210205)
#> [1] 2021
month(210205)
#> [1] 2
quarter(210205)
#> [1] 1
yday(210205)
#> [1] 36
mday(210205)
#> [1] 5
wday(210117)
#> [1] 1
isowday(210117)
#> [1] 7
isoweek(210101)
#> [1] 53
```
