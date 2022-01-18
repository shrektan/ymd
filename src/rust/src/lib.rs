use chrono::NaiveDate;
use extendr_api::prelude::*;
mod dateof;
mod rdate;
use rdate::ToRDate;
mod period;

// short_year: when true, 980102 will be converted to 19980102; when false
// it will be regarded as 00980102
fn int2date(x: i32, short_year: bool) -> Option<NaiveDate> {
    let day: i32 = x % 100;
    let month: i32 = x / 100 % 100;
    let y_part: i32 = x / 10000;
    let year = if short_year && y_part < 100 {
        // 700102 => 19700102; 980403 => 19980403; 691022 =? 20691022; 010204 => 20010204
        if y_part < 70 {
            y_part + 2000
        } else {
            y_part + 1900
        }
    } else {
        y_part
    };
    NaiveDate::from_ymd_opt(year, month as u32, day as u32)
}

fn dbl2date(x: f64) -> Option<NaiveDate> {
    if x % 1.0 == 0.0 {
        int2date(x as i32, true)
    } else {
        None
    }
}

fn str2date(x: &str) -> Option<NaiveDate> {
    match x.parse::<i32>() {
        Ok(v) => int2date(v, true),
        Err(_) => {
            let v: Vec<&str> = x.split(&['-', '.', '/', ' '][..]).collect();
            if v.len() == 3 {
                let short_year = v[0].len() <= 2;
                let year: i32 = v[0].parse().ok()?;
                let month: i32 = v[1].parse().ok()?;
                let day: i32 = v[2].parse().ok()?;
                int2date(year * 10000 + month * 100 + day, short_year)
            } else {
                None
            }
        }
    }
}

#[extendr]
fn rust_ymd(x: Robj) -> Robj {
    if x.inherits("Date") {
        return x;
    }
    let value: Vec<Option<NaiveDate>> = match x.rtype() {
        RType::Integer => x
            .as_integer_iter()
            .unwrap()
            .map(|i| if i.is_na() { None } else { int2date(i, true) })
            .collect(),
        RType::Real => x
            .as_real_iter()
            .unwrap()
            .map(|i| if i.is_na() { None } else { dbl2date(i) })
            .collect(),
        RType::String => x
            .as_str_vector()
            .unwrap()
            .iter()
            .map(|i| if i.is_na() { None } else { str2date(i) })
            .collect(),
        _ => {
            panic!("x must be numeric or string vector");
        }
    };
    value.to_rdate()
}

fn beop(x: Robj, unit: &str, fun: fn(&NaiveDate, period::Period) -> NaiveDate) -> Robj {
    let p = match period::to_period(unit) {
        Some(i) => i,
        None => return (vec![None; x.len()]).to_rdate(),
    };
    let x = rdate::robj2date(rust_ymd(x), "x").unwrap();
    let out: Vec<Option<NaiveDate>> = x
        .iter()
        .map(|v| match v {
            Some(date) => Some(fun(date, p)),
            None => None,
        })
        .collect();
    out.to_rdate()
}

#[extendr]
fn period_begin(x: Robj, unit: &str) -> Robj {
    beop(x, unit, period::bop)
}

#[extendr]
fn period_end(x: Robj, unit: &str) -> Robj {
    beop(x, unit, period::eop)
}

/// Calculate the date before / after months
/// @param ref_date a Date vector
/// @param months the number of months that's added to `ref_date`
/// @note The function name is the same as the Excel function `EDATE()` and
///   does the same. It returns the date that is the indicated number of months
///   before or after the ref date.
/// @examples
/// edate(as.Date("2020-01-31"), 1)
/// ## supports 'YMD' formatted integer or string
/// edate(200131, 1)
/// edate(200229, -12)
///
/// @export
#[extendr]
fn edate(ref_date: Robj, months: i32) -> Robj {
    let out: Vec<Option<NaiveDate>> = rdate::robj2date(rust_ymd(ref_date), "ref_date")
        .unwrap()
        .iter()
        .map(|v| match v {
            Some(date) => Some(period::add_months(date, months)),
            None => None,
        })
        .collect();
    out.to_rdate()
}

macro_rules! make_date_part_fun {
    ($fn_name:ident, $method:expr) => {
        /// @rdname date_part
        /// @export
        #[extendr]
        fn $fn_name(ref_date: Robj) -> Robj {
            let ref_date = rdate::robj2date(rust_ymd(ref_date), "ref_date").unwrap();
            r!($method(&ref_date))
        }
    };
}

make_date_part_fun!(year, dateof::year);
make_date_part_fun!(month, dateof::month);
make_date_part_fun!(quarter, dateof::quarter);
make_date_part_fun!(wday, dateof::wday);
make_date_part_fun!(mday, dateof::mday);
make_date_part_fun!(yday, dateof::yday);
make_date_part_fun!(iso_week, dateof::iso_week);
make_date_part_fun!(iso_wday, dateof::iso_wday);

#[cfg(test)]
mod test {
    use super::*;
    use chrono::NaiveDate;
    #[test]
    fn integers() {
        assert_eq!(
            int2date(980308, true).unwrap(),
            NaiveDate::from_ymd(1998, 3, 8)
        );
        assert_eq!(
            int2date(980308, false).unwrap(),
            NaiveDate::from_ymd(0098, 3, 8)
        );
        assert_eq!(
            int2date(050308, true).unwrap(),
            NaiveDate::from_ymd(2005, 3, 8)
        );
        assert_eq!(
            int2date(19980308, true).unwrap(),
            NaiveDate::from_ymd(1998, 3, 8)
        );
        assert_eq!(
            int2date(21050308, true).unwrap(),
            NaiveDate::from_ymd(2105, 3, 8)
        );
        assert_eq!(int2date(980230, true), None);
        assert_eq!(int2date(19980230, true), None);
        assert_eq!(int2date(22, true), None);
        assert_eq!(
            int2date(2201010, true).unwrap(),
            NaiveDate::from_ymd(0220, 10, 10)
        );
    }

    #[test]
    fn doubles() {
        assert_eq!(dbl2date(980308.).unwrap(), NaiveDate::from_ymd(1998, 3, 8));
        assert_eq!(dbl2date(050308.).unwrap(), NaiveDate::from_ymd(2005, 3, 8));
        assert_eq!(
            dbl2date(19980308.).unwrap(),
            NaiveDate::from_ymd(1998, 3, 8)
        );
        assert_eq!(
            dbl2date(21050308.).unwrap(),
            NaiveDate::from_ymd(2105, 3, 8)
        );
        assert_eq!(dbl2date(980230.), None);
        assert_eq!(dbl2date(19980230.), None);
        assert_eq!(dbl2date(980230.1), None);
        assert_eq!(dbl2date(2201310.), None);
        assert_eq!(dbl2date(220101.5), None);
    }
    #[test]
    fn strings() {
        assert_eq!(str2date("980308").unwrap(), NaiveDate::from_ymd(1998, 3, 8));
        assert_eq!(
            str2date("98.3.08").unwrap(),
            NaiveDate::from_ymd(1998, 3, 8)
        );
        assert_eq!(str2date("98.3.8").unwrap(), NaiveDate::from_ymd(1998, 3, 8));
        assert_eq!(
            str2date("98.03.08").unwrap(),
            NaiveDate::from_ymd(1998, 3, 8)
        );
        assert_eq!(
            str2date("98/03/08").unwrap(),
            NaiveDate::from_ymd(1998, 3, 8)
        );
        assert_eq!(
            str2date("98-03-08").unwrap(),
            NaiveDate::from_ymd(1998, 3, 8)
        );

        assert_eq!(str2date("220102").unwrap(), NaiveDate::from_ymd(2022, 1, 2));
        assert_eq!(
            str2date("22.01.02").unwrap(),
            NaiveDate::from_ymd(2022, 1, 2)
        );
        assert_eq!(
            str2date("22/01/02").unwrap(),
            NaiveDate::from_ymd(2022, 1, 2)
        );
        assert_eq!(
            str2date("22-01-02").unwrap(),
            NaiveDate::from_ymd(2022, 1, 2)
        );

        assert_eq!(
            str2date("19980308").unwrap(),
            NaiveDate::from_ymd(1998, 3, 8)
        );
        assert_eq!(
            str2date("1998.03.08").unwrap(),
            NaiveDate::from_ymd(1998, 3, 8)
        );
        assert_eq!(
            str2date("1998/03/08").unwrap(),
            NaiveDate::from_ymd(1998, 3, 8)
        );
        assert_eq!(
            str2date("1998-03-08").unwrap(),
            NaiveDate::from_ymd(1998, 3, 8)
        );

        assert_eq!(str2date("98308"), None);
        assert_eq!(str2date("980338"), None);
        assert_eq!(str2date("9a0308"), None);
    }
    #[test]
    fn to_date() {
        test! {
            let x: Robj = r!([18990.0, 18991.0]).set_class(&["Date"]).unwrap();
            assert_eq!(rdate::robj2date(x, "x").unwrap(), [Some(NaiveDate::from_ymd(2021, 12, 29)), Some(NaiveDate::from_ymd(2021, 12, 30))]);
        }
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod ymd;
    fn rust_ymd;
    fn period_begin;
    fn period_end;
    fn edate;
    fn year;
    fn month;
    fn quarter;
    fn iso_week;
    fn iso_wday;
    fn wday;
    fn mday;
    fn yday;
}
