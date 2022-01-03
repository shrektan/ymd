use extendr_api::prelude::*;
use chrono::{Datelike, NaiveDate};
mod period;

fn int2date(x: i32) -> Option<NaiveDate> {
    let day: i32 = x % 100;
    let month: i32 = x / 100 % 100;
    let y_part: i32 = x / 10000;
    let year = if y_part < 100 {
        // 700102 => 19700102; 980403 => 19980403; 691022 =? 20691022; 010204 => 20010204
        if y_part < 70 {
            y_part + 2000
        } else {
            y_part + 1900
        }
    } else {
        if y_part < 1000 || y_part > 9999 {
            return None;
        } else {
            y_part
        }
    };
    NaiveDate::from_ymd_opt(year, month as u32, day as u32)
}

fn dbl2date(x: f64) -> Option<NaiveDate> {
    if x % 1.0 == 0.0 {
        int2date(x as i32)
    } else {
        None
    }
}

fn str2date(x: &str) -> Option<NaiveDate> {
    match x.parse::<i32>() {
        Ok(v) => int2date(v),
        Err(_) => {
            let v: Vec<&str> = x.split(&['-', '.', '/', ' '][..]).collect();
            if v.len() == 3 {
                let year: i32 = v[0].parse().ok()?;
                let month: i32 = v[1].parse().ok()?;
                let day: i32 = v[2].parse().ok()?;
                int2date(year * 10000 + month * 100 + day)
            } else {
                None
            }
        },
    }
}

// The days from 1970-1-1 (R's first date) to CE (1-1-0)
const R_DATE_FROM_CE: i32 = 719163;

fn robj2date(x: Robj) -> Vec<Option<NaiveDate>> {
    if !x.inherits("Date") {
        return vec![None; x.len()];
    }
    match x.rtype() {
        RType::Real => {
            x.as_real_iter().unwrap().map(|d| {
                if d.is_na() {
                    None
                } else {
                    NaiveDate::from_num_days_from_ce_opt(d as i32 + R_DATE_FROM_CE)
                }
            })
            .collect()
        },
        RType::Integer => {
            x.as_integer_iter().unwrap().map(|d| {
                if d.is_na() {
                    None
                } else {
                    NaiveDate::from_num_days_from_ce_opt(d + R_DATE_FROM_CE)
                }
            })
            .collect()
        },
        _ => {
            vec![None; x.len()]
        }
    }
}

fn to_rdate(x: &Option<NaiveDate>) -> Option<f64> {
    match x {
        Some(v) => Some((v.num_days_from_ce() - R_DATE_FROM_CE) as f64),
        None => None,
    }
}

fn make_rdate(x: Vec<Option<f64>>) -> Robj {
    r!(x).set_class(&["Date"]).unwrap()
}

fn make_rdate2(x: Vec<Option<NaiveDate>>) -> Robj {
    let v: Vec<Option<f64>> = x.iter().map(to_rdate).collect();
    make_rdate(v)
}

/// Convert integers or strings to Date
///
/// @param x an integerable or string vector in ymd format
///
/// @return a Date object
///
/// @export
#[extendr]
fn ymd(x: Robj) -> Robj {
    if x.inherits("Date") {
        return x;
    }
    let value: Vec<Option<f64>> = match x.rtype() {
        RType::Integer => {
            x.as_integer_iter().unwrap().map(|i| {
                if i.is_na() {
                    None
                } else {
                    to_rdate(&int2date(i))
                }
            })
            .collect()
        },
        RType::Real => {
            x.as_real_iter().unwrap().map(|i| {
                if i.is_na() {
                    None
                } else {
                    to_rdate(&dbl2date(i))
                }
            })
            .collect()
        },
        RType::String => {
            x.as_str_vector().unwrap().iter().map(|i| {
                if i.is_na() {
                    None
                } else {
                    to_rdate(&str2date(i))
                }
            })
            .collect()
        },
        _ => {
            panic!("x must be integerable or string vector");
        }
    };
    make_rdate(value)
}

fn beop(x: Robj, p: &str, fun: fn(&NaiveDate, period::Period)->NaiveDate) -> Robj {
    let p = match period::to_period(p) {
        Some(i) => i,
        None => return make_rdate( vec![None; x.len()] ),
    };
    let x = robj2date(x);
    let out = x.iter().map(|v| {
        match v {
            Some(date) => Some(fun(date, p)),
            None => None,
        }
    }).collect();
    make_rdate2(out)
}

#[extendr]
fn period_begin(x: Robj, p: &str) -> Robj {
    beop(x, p, period::bop)
}

#[extendr]
fn period_end(x: Robj, p: &str) -> Robj {
    beop(x, p, period::eop)
}

/// Add months to a Date
/// @param x a Date vector
/// @param n the number of months that's added to `x`
/// @export
#[extendr]
fn add_months(x: Robj, n: i32) -> Robj {
    let x = robj2date(x);
    let out = x.iter().map(|v| {
        match v {
            Some(date) => Some(period::add_months(date, n)),
            None => None,
        }
    })
    .collect();
    make_rdate2(out)
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::{NaiveDate};
    #[test]
    fn integers() {
        assert_eq!(int2date(980308).unwrap(), NaiveDate::from_ymd(1998, 3, 8));
        assert_eq!(int2date(050308).unwrap(), NaiveDate::from_ymd(2005, 3, 8));
        assert_eq!(int2date(19980308).unwrap(), NaiveDate::from_ymd(1998, 3, 8));
        assert_eq!(int2date(21050308).unwrap(), NaiveDate::from_ymd(2105, 3, 8));
        assert_eq!(int2date(980230), None);
        assert_eq!(int2date(19980230), None);
        assert_eq!(int2date(22), None);
        assert_eq!(int2date(2201010), None);
    }

    #[test]
    fn doubles() {
        assert_eq!(dbl2date(980308.).unwrap(), NaiveDate::from_ymd(1998, 3, 8));
        assert_eq!(dbl2date(050308.).unwrap(), NaiveDate::from_ymd(2005, 3, 8));
        assert_eq!(dbl2date(19980308.).unwrap(), NaiveDate::from_ymd(1998, 3, 8));
        assert_eq!(dbl2date(21050308.).unwrap(), NaiveDate::from_ymd(2105, 3, 8));
        assert_eq!(dbl2date(980230.), None);
        assert_eq!(dbl2date(19980230.), None);
        assert_eq!(dbl2date(980230.1), None);
        assert_eq!(dbl2date(2201010.), None);
        assert_eq!(dbl2date(220101.5), None);
    }
    #[test]
    fn strings() {
        assert_eq!(str2date("980308").unwrap(), NaiveDate::from_ymd(1998, 3, 8));
        assert_eq!(str2date("98.3.08").unwrap(), NaiveDate::from_ymd(1998, 3, 8));
        assert_eq!(str2date("98.3.8").unwrap(), NaiveDate::from_ymd(1998, 3, 8));
        assert_eq!(str2date("98.03.08").unwrap(), NaiveDate::from_ymd(1998, 3, 8));
        assert_eq!(str2date("98/03/08").unwrap(), NaiveDate::from_ymd(1998, 3, 8));
        assert_eq!(str2date("98-03-08").unwrap(), NaiveDate::from_ymd(1998, 3, 8));

        assert_eq!(str2date("220102").unwrap(), NaiveDate::from_ymd(2022, 1, 2));
        assert_eq!(str2date("22.01.02").unwrap(), NaiveDate::from_ymd(2022, 1, 2));
        assert_eq!(str2date("22/01/02").unwrap(), NaiveDate::from_ymd(2022, 1, 2));
        assert_eq!(str2date("22-01-02").unwrap(), NaiveDate::from_ymd(2022, 1, 2));

        assert_eq!(str2date("19980308").unwrap(), NaiveDate::from_ymd(1998, 3, 8));
        assert_eq!(str2date("1998.03.08").unwrap(), NaiveDate::from_ymd(1998, 3, 8));
        assert_eq!(str2date("1998/03/08").unwrap(), NaiveDate::from_ymd(1998, 3, 8));
        assert_eq!(str2date("1998-03-08").unwrap(), NaiveDate::from_ymd(1998, 3, 8));

        assert_eq!(str2date("98308"), None);
        assert_eq!(str2date("9800308"), None);
        assert_eq!(str2date("9a0308"), None);
    }
    #[test]
    fn to_date() {
        test! {
            let x: Robj = r!([18990.0, 18991.0]).set_class(&["Date"]).unwrap();
            assert_eq!(robj2date(x), [Some(NaiveDate::from_ymd(2021, 12, 29)), Some(NaiveDate::from_ymd(2021, 12, 30))]);
        }
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod ymd;
    fn ymd;
    fn period_begin;
    fn period_end;
    fn add_months;
}
