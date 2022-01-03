#' Begin/End of Period (BOP and EOP)
#'
#' A list of BOP/EOP functions.
#'
#' @param x a date vector
#' @note `bop` and `eop` provides a list of functions. Their names consist of two
#'   letters, the first which means __l__ast, __t__his, __n__ext while the second
#'   means __y__ear, __q__uarter, __m__onth, __w__eek. Thus, `eop$ty()` means
#'   "the __e__nd __o__f period of __t__his __y__ear".
#'
#' @name beop
#' @export
eop <- list(
  ly = function(x = Sys.Date()) period_begin(x, 'year') - 1L,
  ty = function(x = Sys.Date()) period_end(x, 'year'),
  ny = function(x = Sys.Date()) period_end(period_end(x, 'year') + 1L, 'year'),
  lq = function(x = Sys.Date()) period_begin(x, 'quarter') - 1L,
  tq = function(x = Sys.Date()) period_end(x, 'quarter'),
  nq = function(x = Sys.Date()) period_end(period_end(x, 'quarter') + 1L, 'quarter'),
  lm = function(x = Sys.Date()) period_begin(x, 'month') - 1L,
  tm = function(x = Sys.Date()) period_end(x, 'month'),
  nm = function(x = Sys.Date()) period_end(period_end(x, 'month') + 1L, 'month'),
  lw = function(x = Sys.Date()) period_begin(x, 'week') - 1L,
  tw = function(x = Sys.Date()) period_end(x, 'week'),
  nw = function(x = Sys.Date()) period_end(period_end(x, 'week') + 1L, 'week')
)

#' @name beop
#' @export
bop <- list(
  ly = function(x = Sys.Date()) period_begin(period_begin(x, 'year') - 1L, 'year'),
  ty = function(x = Sys.Date()) period_begin(x, 'year'),
  ny = function(x = Sys.Date()) period_end(x, 'year') + 1L,
  lq = function(x = Sys.Date()) period_begin(period_begin(x, 'quarter') - 1L, 'quarter'),
  tq = function(x = Sys.Date()) period_begin(x, 'quarter'),
  nq = function(x = Sys.Date()) period_end(x, 'quarter') + 1L,
  lm = function(x = Sys.Date()) period_begin(period_begin(x, 'month') - 1L, 'month'),
  tm = function(x = Sys.Date()) period_begin(x, 'month'),
  nm = function(x = Sys.Date()) period_end(x, 'month') + 1L,
  lw = function(x = Sys.Date()) period_begin(period_begin(x, 'week') - 1L, 'week'),
  tw = function(x = Sys.Date()) period_begin(x, 'week'),
  nw = function(x = Sys.Date()) period_end(x, 'week') + 1L
)
