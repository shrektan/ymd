#' Convert 'YMD' format integer or string to Date
#'
#' Transform integer or strings vectors in 'YMD' format to Date objects.
#' It intends to only support limited formats (no separator or one of
#' '.', ' ', '-' and '/' separators). See the possible formats in examples.
#'
#' @param x An integer or string vector in 'YMD' format. Double
#'   values without the decimal part are allowed.
#' @param ... The same as `x`. It will be merged into one vector with `x`.
#'   It's convinient for interactive use.
#'
#' @return A Date object. When the parse fails for certain input,
#'   the value returned would be `NA`, silently.
#'
#' @examples
#' ymd(c(210326, 19981225))
#' ymd(c("2020/1/8", "20 1 7", "1998.7.1", "1990-02-03"))
#' ymd(210420, 180322)
#'
#' @export
ymd <- function(x, ...) {
  if (...length()) {
    x <- c(x, unlist(list(...)))
  }
  rust_ymd(x)
}
