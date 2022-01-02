use extendr_api::prelude::*;
use chrono::{Datelike, NaiveDate};

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
    if x.len() < 6 || x.len() > 10 || x.len() % 2 != 0 {
        return None;
    }
    const FMTS: [&str;3] = ["-", ".", "/"];
    let mut v: String = String::from(x);
    for fmt in &FMTS {
        v = v.replace(fmt, "");
    };
    let v: i32 = v.parse().ok()?;
    int2date(v)
}

fn to_rdate(x: &Option<NaiveDate>) -> Option<f64> {
    match x {
        Some(v) => Some(v.num_days_from_ce() as f64 - 719163.0),
        None => None,
    }
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
}


// /// Convert integers or strings to Date
// ///
// /// @param x an integerable or string vector in ymd format
// ///
// /// @value a Date object
// ///
// /// @export
// #[extendr]
// fn ymd(x: Robj) -> Robj {
//     if x.inherits("Date") {
//         return x;
//     }
//     let out: NaiveDate = match x.rtype() {
//         RType::Integer => {

//         },
//         RType::Real => {

//         },
//         RType::Character => {

//         },
//         _ => {
//             panic!("x must be integerable or string vector");
//         }
//     };
// }

// // Macro to generate exports.
// // This ensures exported functions are registered with R.
// // See corresponding C code in `entrypoint.c`.
// extendr_module! {
//     mod ymd;
//     fn ymd;
// }
