#' Find the Beginning or Ending of Period
#'
#' Each of `bop` and `eop` contains a list of functions, whose names all
#' consist of two letters, the first of which stands for **l**ast, **t**his, **n**ext
#' while the second stands for **y**ear, **q**uarter, **m**onth, **w**eek.
#' For example, `eop$ty()` means "the **e**nding **o**f **p**eriod of **t**his **y**ear"
#' and `bop$lm()` means "the **b**eginning **o**f **p**eriod of **l**ast **m**onth".
#'
#' @rdname beop
#' @export
eop <- list(
  ly = function(x = Sys.Date()) period_begin(x, 'year') - 1,
  ty = function(x = Sys.Date()) period_end(x, 'year'),
  ny = function(x = Sys.Date()) period_end(period_end(x, 'year') + 1, 'year'),
  lq = function(x = Sys.Date()) period_begin(x, 'quarter') - 1,
  tq = function(x = Sys.Date()) period_end(x, 'quarter'),
  nq = function(x = Sys.Date()) period_end(period_end(x, 'quarter') + 1, 'quarter'),
  lm = function(x = Sys.Date()) period_begin(x, 'month') - 1,
  tm = function(x = Sys.Date()) period_end(x, 'month'),
  nm = function(x = Sys.Date()) period_end(period_end(x, 'month') + 1, 'month'),
  lw = function(x = Sys.Date()) period_begin(x, 'week') - 1,
  tw = function(x = Sys.Date()) period_end(x, 'week'),
  nw = function(x = Sys.Date()) period_end(period_end(x, 'week') + 1, 'week')
)

#' @rdname beop
#' @export
bop <- list(
  ly = function(x = Sys.Date()) period_begin(period_begin(x, 'year') - 1, 'year'),
  ty = function(x = Sys.Date()) period_begin(x, 'year'),
  ny = function(x = Sys.Date()) period_end(x, 'year') + 1,
  lq = function(x = Sys.Date()) period_begin(period_begin(x, 'quarter') - 1, 'quarter'),
  tq = function(x = Sys.Date()) period_begin(x, 'quarter'),
  nq = function(x = Sys.Date()) period_end(x, 'quarter') + 1,
  lm = function(x = Sys.Date()) period_begin(period_begin(x, 'month') - 1, 'month'),
  tm = function(x = Sys.Date()) period_begin(x, 'month'),
  nm = function(x = Sys.Date()) period_end(x, 'month') + 1,
  lw = function(x = Sys.Date()) period_begin(period_begin(x, 'week') - 1, 'week'),
  tw = function(x = Sys.Date()) period_begin(x, 'week'),
  nw = function(x = Sys.Date()) period_end(x, 'week') + 1
)
