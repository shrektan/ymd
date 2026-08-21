# Convert 'YMD' format integer or string to Date

Transform integer or strings vectors in 'YMD' format to Date objects. It
intends to only support limited formats (no separator or one of '.', '
', '-' and '/' separators). See the possible formats in examples.

## Usage

``` r
ymd(x, ...)
```

## Arguments

- x:

  An integer or string vector in 'YMD' format. Double values without the
  decimal part are allowed.

- ...:

  The same as `x`. It will be merged into one vector with `x`. It's
  convenient for interactive use.

## Value

A Date object. When the parse fails for certain input, the value
returned would be `NA`, silently.

## Examples

``` r
ymd(c(210326, 19981225))
#> [1] "2021-03-26" "1998-12-25"
ymd(c("2020/1/8", "20 1 7", "1998.7.1", "1990-02-03"))
#> [1] "2020-01-08" "2020-01-07" "1998-07-01" "1990-02-03"
ymd(210420, 180322)
#> [1] "2021-04-20" "2018-03-22"
```
