test_that("date helpers follow R semantics for fractional Dates", {
  dates <- .Date(c(-1.5, -0.5, 0.5, 1.5))

  expect_equal(year(dates), c(1969L, 1969L, 1970L, 1970L))
  expect_equal(month(dates), c(12L, 12L, 1L, 1L))
  expect_equal(quarter(dates), c(4L, 4L, 1L, 1L))
  expect_equal(wday(dates), c(3L, 4L, 5L, 6L))
  expect_equal(mday(dates), c(30L, 31L, 1L, 2L))
  expect_equal(yday(dates), c(364L, 365L, 1L, 2L))
  expect_equal(isoweek(dates), rep(1L, 4L))
  expect_equal(isowday(dates), c(2L, 3L, 4L, 5L))
})

test_that("date operations follow R semantics for fractional Dates", {
  dates <- .Date(c(-1.5, -0.5, 0.5, 1.5))

  expect_equal(edate(dates, 0L), .Date(c(-2, -1, 0, 1)))
  expect_equal(bop$tm(dates), as.Date(c("1969-12-01", "1969-12-01", "1970-01-01", "1970-01-01")))
  expect_equal(eop$tm(dates), as.Date(c("1969-12-31", "1969-12-31", "1970-01-31", "1970-01-31")))
})

test_that("non-finite Dates return missing date parts", {
  dates <- .Date(c(NA_real_, NaN, Inf, -Inf))

  expect_equal(mday(dates), rep(NA_integer_, 4L))
  expect_equal(edate(dates, 0L), .Date(rep(NA_real_, 4L)))
})

test_that("out-of-range Dates return missing date parts", {
  dates <- .Date(c(1e20, -1e20))

  expect_equal(mday(dates), rep(NA_integer_, 2L))
  expect_equal(edate(dates, 0L), .Date(rep(NA_real_, 2L)))
})
