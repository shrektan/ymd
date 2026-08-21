# Calculate the date before / after months

Calculate the date before / after months

## Usage

``` r
edate(ref_date, months)
```

## Arguments

- ref_date:

  a Date vector

- months:

  the number of months that's added to `ref_date`

## Value

A Date vector with `months` added to `ref_date`.

## Note

The function name is the same as the Excel function `EDATE()` and does
the same. It returns the date that is the indicated number of months
before or after the ref date.

## Examples

``` r
edate(as.Date("2020-01-31"), 1)
#> [1] "2020-02-29"
## supports 'YMD' formatted integer or string
edate(200131, 1)
#> [1] "2020-02-29"
edate(200229, -12)
#> [1] "2019-02-28"
```
