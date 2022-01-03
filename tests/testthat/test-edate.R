test_that("edate works", {
  expect_identical(edate(210131, 0), ymd(210131))
  expect_identical(edate(210131, 1), ymd(210228))
  expect_identical(edate(200131, 1), ymd(200229)) # leap year
  expect_identical(edate(210531, -1), ymd(210430))
  expect_identical(edate(210531, -3), ymd(210228))
  expect_identical(edate(200531, -3), ymd(200229)) # leap year
  expect_identical(edate(210515, -8), ymd(200915))
  expect_identical(edate(210515, 8), ymd(220115))
  expect_identical(edate(200229, -12), ymd(190228))
  expect_identical(edate(200229, 12), ymd(210228))
})
