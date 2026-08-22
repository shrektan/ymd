use chrono::{Datelike, NaiveDate};
use extendr_api::prelude::*;

// The days from 1970-1-1 (R's first date) to CE (1-1-0)
const R_DATE_FROM_CE: i32 = 719163;

fn days_since_epoch_to_date(days: i32) -> Option<NaiveDate> {
    days.checked_add(R_DATE_FROM_CE)
        .and_then(NaiveDate::from_num_days_from_ce_opt)
}

fn real_days_since_epoch_to_date(days: f64) -> Option<NaiveDate> {
    if !days.is_finite() {
        return None;
    }

    let days = days.floor();
    if days < i32::MIN as f64 || days > i32::MAX as f64 {
        return None;
    }

    days_since_epoch_to_date(days as i32)
}

pub fn robj2date(x: Robj, var: &str) -> extendr_api::Result<Vec<Option<NaiveDate>>> {
    if !x.inherits("Date") {
        return Err(Error::Other(format!("{} is not a Date", var)));
    }
    let out = match x.rtype() {
        Rtype::Doubles => x
            .as_real_iter()
            .unwrap()
            .map(|d| real_days_since_epoch_to_date(*d))
            .collect(),
        Rtype::Integers => x
            .as_integer_vector()
            .unwrap()
            .iter()
            .map(|d| {
                if d.is_na() {
                    None
                } else {
                    days_since_epoch_to_date(*d)
                }
            })
            .collect(),
        _ => {
            return Err(Error::Other(format!(
                "{} is Date but the type is not integer or double",
                var
            )));
        }
    };
    Ok(out)
}

fn date2rnum(x: &NaiveDate) -> f64 {
    (x.num_days_from_ce() - R_DATE_FROM_CE) as f64
}

pub trait ToRDate {
    fn to_rdate(&self) -> Robj;
}

impl ToRDate for [Option<NaiveDate>] {
    fn to_rdate(&self) -> Robj {
        let out: Vec<Option<f64>> = self.iter().map(|v| v.as_ref().map(date2rnum)).collect();
        out.to_rdate()
    }
}

impl ToRDate for [NaiveDate] {
    fn to_rdate(&self) -> Robj {
        let out: Vec<f64> = self.iter().map(date2rnum).collect();
        out.to_rdate()
    }
}

impl ToRDate for Vec<Option<f64>> {
    fn to_rdate(&self) -> Robj {
        r!(self.clone()).set_class(&["Date"]).unwrap().clone()
    }
}

impl ToRDate for [f64] {
    fn to_rdate(&self) -> Robj {
        r!(self).set_class(&["Date"]).unwrap().clone()
    }
}

impl ToRDate for [i32] {
    fn to_rdate(&self) -> Robj {
        let out: Vec<f64> = self.iter().map(|v| *v as f64).collect();
        r!(out).set_class(&["Date"]).unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn to_date() {
        test! {
            single_threaded(|| {
                let r_dates: Robj = r!([18990.0, 18991.0]).set_class(&["Date"]).unwrap().clone();
                let rust_dates = [Some(NaiveDate::from_ymd_opt(2021, 12, 29).unwrap()), Some(NaiveDate::from_ymd_opt(2021, 12, 30).unwrap())];
                assert_eq!(robj2date(r_dates.clone(), "r_dates").unwrap(), rust_dates);
                assert_eq!(rust_dates.to_rdate(), r_dates);
                let rust_dates = Vec::from([Some(NaiveDate::from_ymd_opt(2021, 12, 29).unwrap()), Some(NaiveDate::from_ymd_opt(2021, 12, 30).unwrap())]);
                assert_eq!(rust_dates.to_rdate(), r_dates);
                let rust_dates = [NaiveDate::from_ymd_opt(2021, 12, 29).unwrap(), NaiveDate::from_ymd_opt(2021, 12, 30).unwrap()];
                assert_eq!(rust_dates.to_rdate(), r_dates);
                let rust_dates = Vec::from([NaiveDate::from_ymd_opt(2021, 12, 29).unwrap(), NaiveDate::from_ymd_opt(2021, 12, 30).unwrap()]);
                assert_eq!(rust_dates.to_rdate(), r_dates);
            });
        }
    }

    #[test]
    fn floor_fractional_days() {
        assert_eq!(
            real_days_since_epoch_to_date(-1.5),
            NaiveDate::from_ymd_opt(1969, 12, 30)
        );
        assert_eq!(
            real_days_since_epoch_to_date(-0.5),
            NaiveDate::from_ymd_opt(1969, 12, 31)
        );
        assert_eq!(
            real_days_since_epoch_to_date(0.5),
            NaiveDate::from_ymd_opt(1970, 1, 1)
        );
        assert_eq!(
            real_days_since_epoch_to_date(1.5),
            NaiveDate::from_ymd_opt(1970, 1, 2)
        );
    }

    #[test]
    fn non_finite_and_out_of_range_days() {
        assert_eq!(real_days_since_epoch_to_date(f64::NAN), None);
        assert_eq!(real_days_since_epoch_to_date(f64::INFINITY), None);
        assert_eq!(real_days_since_epoch_to_date(f64::NEG_INFINITY), None);
        assert_eq!(real_days_since_epoch_to_date(1e20), None);
        assert_eq!(real_days_since_epoch_to_date(-1e20), None);
        assert_eq!(days_since_epoch_to_date(i32::MAX), None);
    }
}
