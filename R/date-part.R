#' Extract Date Part
#'
#' @param ref_date, a Date vector. It will try to convert the input to date via [ymd()],
#'   if the input is not a Date.
#' @return an integer vector
#' @section Detailed explanation:
#'   * year, month, quarter: get the year, month, quarter part
#'   * iso_week: ISO 8601 week starting from 1
#'   * mday: the day of month starting from 1
#'   * yday: the day of year starting from 1
#'   * wday: the day of week (ISO 8601 weekday number, Monday is 1)
#' @name date_part
NULL
