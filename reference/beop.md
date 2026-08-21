# Find the Beginning or End of Period

Each of `bop` and `eop` contains a list of functions, whose names all
consist of two letters, the first of which stands for **l**ast,
**t**his, **n**ext while the second stands for **y**ear, **q**uarter,
**m**onth, **w**eek. For example, `eop$ty()` means "the **e**nding
**o**f **p**eriod of **t**his **y**ear" and `bop$lm()` means "the
**b**eginning **o**f **p**eriod of **l**ast **m**onth".

## Details

All functions' signatures are the same, with only one argument `x`,
which could be a `Date` or values that can be converted to `Date` via
[`ymd()`](https://shrektan.github.io/ymd/reference/ymd.md).

## Examples

``` r
bop$ty(as.Date("2021-03-02"))
#> [1] "2021-01-01"
## supports 'YMD' formatted integer or string
bop$ty(210302)
#> [1] "2021-01-01"
eop$tm(200201)
#> [1] "2020-02-29"
```
