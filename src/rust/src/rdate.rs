use chrono::{Datelike, NaiveDate};
use extendr_api::prelude::*;

// The days from 1970-1-1 (R's first date) to CE (1-1-0)
const R_DATE_FROM_CE: i32 = 719163;

pub fn robj2date(x: Robj, var: &str) -> Result<Vec<Option<NaiveDate>>> {
    if !x.inherits("Date") {
        return Err(Error::Other(format!("{} is not a Date", var)));
    }
    let out = match x.rtype() {
        RType::Real => x
            .as_real_iter()
            .unwrap()
            .map(|d| {
                if d.is_na() {
                    None
                } else {
                    NaiveDate::from_num_days_from_ce_opt(d as i32 + R_DATE_FROM_CE)
                }
            })
            .collect(),
        RType::Integer => x
            .as_integer_iter()
            .unwrap()
            .map(|d| {
                if d.is_na() {
                    None
                } else {
                    NaiveDate::from_num_days_from_ce_opt(d + R_DATE_FROM_CE)
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
        let out: Vec<Option<f64>> = self
            .iter()
            .map(|v| match v {
                Some(v) => Some(date2rnum(v)),
                None => None,
            })
            .collect();
        out.to_rdate()
    }
}

impl ToRDate for [NaiveDate] {
    fn to_rdate(&self) -> Robj {
        let out: Vec<f64> = self.iter().map(|v| date2rnum(v)).collect();
        out.to_rdate()
    }
}

impl ToRDate for Vec<Option<f64>> {
    fn to_rdate(&self) -> Robj {
        r!(self.clone()).set_class(&["Date"]).unwrap()
    }
}

impl ToRDate for [f64] {
    fn to_rdate(&self) -> Robj {
        r!(self).set_class(&["Date"]).unwrap()
    }
}

impl ToRDate for [i32] {
    fn to_rdate(&self) -> Robj {
        let out: Vec<f64> = self.iter().map(|v| *v as f64).collect();
        r!(out).set_class(&["Date"]).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn to_date() {
        test! {
            single_threaded(|| {
                let r_dates: Robj = r!([18990.0, 18991.0]).set_class(&["Date"]).unwrap();
                let rust_dates = [Some(NaiveDate::from_ymd(2021, 12, 29)), Some(NaiveDate::from_ymd(2021, 12, 30))];
                assert_eq!(robj2date(r_dates.clone(), "r_dates").unwrap(), rust_dates);
                assert_eq!(rust_dates.to_rdate(), r_dates);
                let rust_dates = vec![Some(NaiveDate::from_ymd(2021, 12, 29)), Some(NaiveDate::from_ymd(2021, 12, 30))];
                assert_eq!(rust_dates.to_rdate(), r_dates);
                let rust_dates = [NaiveDate::from_ymd(2021, 12, 29), NaiveDate::from_ymd(2021, 12, 30)];
                assert_eq!(rust_dates.to_rdate(), r_dates);
                let rust_dates = vec![NaiveDate::from_ymd(2021, 12, 29), NaiveDate::from_ymd(2021, 12, 30)];
                assert_eq!(rust_dates.to_rdate(), r_dates);
            });
        }
    }
}
