#' Fast Date Part Extracting
#'
#' These date helper functions provide the similar functionalities like in `data.table` or
#' `lubridate` package. They are implemented by the Rust Lang's standard library and very
#' fast.
#'
#' @param ref_date, a Date vector. It will try to convert the input to date via [ymd()],
#'   if the input is not a Date.
#' @return an integer vector
#' @details
#'   * year, month, quarter: get the year, month, quarter part
#'   * yday: the day of year
#'   * mday: the day of month
#'   * wday: the day of the week (Sunday is 1)
#'   * isoweek: ISO 8601 week
#'   * isowday: the day of week (ISO 8601 weekday number, Monday is 1)
#' @references
#' ISO week day, https://en.wikipedia.org/wiki/ISO_week_date
#' ISO 8601, https://en.wikipedia.org/wiki/ISO_8601
#' @examples
#' year(210205)
#' month(210205)
#' quarter(210205)
#' yday(210205)
#' mday(210205)
#' wday(210117)
#' isowday(210117)
#' isoweek(210101)
#'
#' @name date_part
NULL
