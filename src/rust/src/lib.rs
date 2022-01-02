use extendr_api::prelude::*;
use chrono::{Datelike, NaiveDate, Weekday};

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

fn to_rdate(x: &Option<NaiveDate>) -> Option<f64> {
    match x {
        Some(v) => Some(v.num_days_from_ce() as f64 - 719163.0),
        None => None,
    }
}

fn add_days(ref_date: &NaiveDate, days: i32) -> NaiveDate {
    NaiveDate::from_num_days_from_ce(ref_date.num_days_from_ce() + days)
}

fn add_months(ref_date: &NaiveDate, months: i32) -> NaiveDate {
    let num_of_months = ref_date.year() * 12 + ref_date.month() as i32 + months as i32;
    let year = (num_of_months - 1) / 12;
    let month = (num_of_months - 1) % 12 + 1;
    let since = NaiveDate::signed_duration_since;
    let nxt_month = if month == 12 {
        NaiveDate::from_ymd(year + 1, 1 as u32, 1)
    } else {
        NaiveDate::from_ymd(year, (month + 1) as u32, 1)
    };
    let max_day =
        since(nxt_month, NaiveDate::from_ymd(year, month as u32, 1)).num_days() as u32;
    let day = ref_date.day();
    NaiveDate::from_ymd(
        year,
        month as u32,
        if day > max_day { max_day } else { day },
    )
}

enum Period {
    Year,
    Semiannual,
    Quarter,
    Month,
    Week,
}

fn bop(x: &NaiveDate, p: Period) -> NaiveDate {
    match p {
        Period::Year => {
            NaiveDate::from_ymd(x.year(), 1, 1)
        },
        Period::Semiannual => {
            let month = match x.month() {
                1..=6 => 1,
                _ => 7,
            };
            NaiveDate::from_ymd(x.year(), month, 1)
        },
        Period::Quarter => {
            let month = match x.month() {
                1..=3 => 1,
                4..=6 => 4,
                7..=9 => 7,
                _ => 10,
            };
            NaiveDate::from_ymd(x.year(), month, 1)
        },
        Period::Month => {
            NaiveDate::from_ymd(x.year(), x.month(), 1)
        },
        Period::Week => {
            NaiveDate::from_isoywd(x.iso_week().year(), x.iso_week().week(), Weekday::Mon)
        },
    }
}

fn eop(x: &NaiveDate, p: Period) -> NaiveDate {
    match p {
        Period::Year => {
            NaiveDate::from_ymd(x.year(), 12, 31)
        },
        Period::Semiannual => {
            match x.month() {
                1..=6 => NaiveDate::from_ymd(x.year(), 6, 30),
                _ => NaiveDate::from_ymd(x.year(), 12, 31),
            }
        },
        Period::Quarter => {
            match x.month() {
                1..=3 => NaiveDate::from_ymd(x.year(), 3, 31),
                4..=6 => NaiveDate::from_ymd(x.year(), 6, 30),
                7..=9 => NaiveDate::from_ymd(x.year(), 9, 30),
                _ => NaiveDate::from_ymd(x.year(), 12, 31)
            }
        },
        Period::Month => {
            let bop = NaiveDate::from_ymd(x.year(), x.month(), 1);
            add_days(&add_months(&bop, 1), -1)
        },
        Period::Week => {
            NaiveDate::from_isoywd(x.iso_week().year(), x.iso_week().week(), Weekday::Sun)
        },
    }
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
    let out: Robj = r!(value).set_class(&["Date"]).unwrap();
    out
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
    fn test_add_days() {
        let fromymd = NaiveDate::from_ymd;
        assert_eq!(
            add_days(&fromymd(2021, 1, 1), -1),
            fromymd(2020, 12, 31)
        );
        assert_eq!(
            add_days(&fromymd(2021, 1, 31), 1),
            fromymd(2021, 2, 1)
        );
        assert_eq!(
            add_days(&fromymd(2021, 12, 31), 1),
            fromymd(2022, 1, 1)
        );
    }
    #[test]
    fn test_add_months() {
        let fromymd = NaiveDate::from_ymd;
        assert_eq!(
            add_months(&fromymd(2021, 1, 1), -1),
            fromymd(2020, 12, 1)
        );
        assert_eq!(
            add_months(&fromymd(2021, 1, 31), 1),
            fromymd(2021, 2, 28)
        );
        assert_eq!(
            add_months(&fromymd(2021, 12, 31), 12),
            fromymd(2022, 12, 31)
        );
    }
    #[test]
    fn test_bop() {
        let fromymd = NaiveDate::from_ymd;
        assert_eq!(
            bop(&fromymd(2021, 1, 15), Period::Year),
            fromymd(2021, 1, 1)
        );
        assert_eq!(
            bop(&fromymd(2021, 12, 15), Period::Semiannual),
            fromymd(2021, 7, 1)
        );
        assert_eq!(
            bop(&fromymd(2021, 5, 15), Period::Quarter),
            fromymd(2021, 4, 1)
        );
        assert_eq!(
            bop(&fromymd(2021, 8, 31), Period::Month),
            fromymd(2021, 8, 1)
        );
        assert_eq!(
            bop(&fromymd(2022, 1, 1), Period::Week),
            fromymd(2021, 12, 27)
        );
    }
    #[test]
    fn test_eop() {
        let fromymd = NaiveDate::from_ymd;
        assert_eq!(
            eop(&fromymd(2021, 1, 15), Period::Year),
            fromymd(2021, 12, 31)
        );
        assert_eq!(
            eop(&fromymd(2021, 1, 15), Period::Semiannual),
            fromymd(2021, 6, 30)
        );
        assert_eq!(
            eop(&fromymd(2021, 5, 15), Period::Quarter),
            fromymd(2021, 6, 30)
        );
        assert_eq!(
            eop(&fromymd(2021, 2, 12), Period::Month),
            fromymd(2021, 2, 28)
        );
        assert_eq!(
            eop(&fromymd(2022, 1, 1), Period::Week),
            fromymd(2022, 1, 2)
        );
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod ymd;
    fn ymd;
}
