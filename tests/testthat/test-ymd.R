test_that("ymd works", {
  expect_equal(ymd(980112), as.Date("1998-01-12"))
  expect_equal(ymd(19980112), as.Date("1998-01-12"))
  expect_equal(ymd(210112), as.Date("2021-01-12"))
  expect_equal(ymd(20200112), as.Date("2020-01-12"))

  expect_equal(ymd(980112L), as.Date("1998-01-12"))
  expect_equal(ymd(19980112L), as.Date("1998-01-12"))
  expect_equal(ymd(210112L), as.Date("2021-01-12"))
  expect_equal(ymd(20200112L), as.Date("2020-01-12"))


  expect_equal(ymd("980112"), as.Date("1998-01-12"))
  expect_equal(ymd("19980112"), as.Date("1998-01-12"))
  expect_equal(ymd("210112"), as.Date("2021-01-12"))
  expect_equal(ymd("20200112"), as.Date("2020-01-12"))

  expect_equal(ymd("98/01/12"), as.Date("1998-01-12"))
  expect_equal(ymd("1998/01/12"), as.Date("1998-01-12"))
  expect_equal(ymd("21/01/12"), as.Date("2021-01-12"))
  expect_equal(ymd("2020/01/12"), as.Date("2020-01-12"))

  expect_equal(ymd("98/1/12"), as.Date("1998-01-12"))
  expect_equal(ymd("1998/1/12"), as.Date("1998-01-12"))
  expect_equal(ymd("21/1/12"), as.Date("2021-01-12"))
  expect_equal(ymd("2020/1/1"), as.Date("2020-01-01"))

  expect_equal(ymd("98-01-12"), as.Date("1998-01-12"))
  expect_equal(ymd("1998-01-12"), as.Date("1998-01-12"))
  expect_equal(ymd("21-01-12"), as.Date("2021-01-12"))
  expect_equal(ymd("2020-01-12"), as.Date("2020-01-12"))

  expect_equal(ymd(980112.1), structure(NA_real_, class = "Date"))
  expect_equal(ymd("1998//01/1"), structure(NA_real_, class = "Date"))
})

test_that("parse short year dates correctly", {
  expect_equal(ymd("0098-03-05"), as.Date("0098-03-05"))
  expect_equal(ymd("98-3-05"), as.Date("1998-03-05"))
})

test_that("ymd ... works", {
  expect_equal(ymd(210101, 220101), ymd(c(210101, 220101)))
})
