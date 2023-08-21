use chrono::{Datelike, NaiveDate, Weekday};

pub fn add_days(ref_date: &NaiveDate, days: i32) -> NaiveDate {
    NaiveDate::from_num_days_from_ce_opt(ref_date.num_days_from_ce() + days).unwrap()
}

pub fn add_months(ref_date: &NaiveDate, months: i32) -> NaiveDate {
    let num_of_months = ref_date.year() * 12 + ref_date.month() as i32 + months as i32;
    let year = (num_of_months - 1) / 12;
    let month = (num_of_months - 1) % 12 + 1;
    let since = NaiveDate::signed_duration_since;
    let nxt_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1 as u32, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(year, (month + 1) as u32, 1).unwrap()
    };
    let max_day = since(
        nxt_month,
        NaiveDate::from_ymd_opt(year, month as u32, 1).unwrap(),
    )
    .num_days() as u32;
    let day = ref_date.day();
    NaiveDate::from_ymd_opt(
        year,
        month as u32,
        if day > max_day { max_day } else { day },
    )
    .unwrap()
}

#[derive(Copy, Clone)]
pub enum Period {
    Year,
    Semiannual,
    Quarter,
    Month,
    Week,
}

pub fn to_period(x: &str) -> Option<Period> {
    match x {
        "year" => Some(Period::Year),
        "semiannual" => Some(Period::Semiannual),
        "quarter" => Some(Period::Quarter),
        "month" => Some(Period::Month),
        "week" => Some(Period::Week),
        _ => None,
    }
}

pub fn bop(x: &NaiveDate, p: Period) -> NaiveDate {
    match p {
        Period::Year => NaiveDate::from_ymd_opt(x.year(), 1, 1).unwrap(),
        Period::Semiannual => {
            let month = match x.month() {
                1..=6 => 1,
                _ => 7,
            };
            NaiveDate::from_ymd_opt(x.year(), month, 1).unwrap()
        }
        Period::Quarter => {
            let month = match x.month() {
                1..=3 => 1,
                4..=6 => 4,
                7..=9 => 7,
                _ => 10,
            };
            NaiveDate::from_ymd_opt(x.year(), month, 1).unwrap()
        }
        Period::Month => NaiveDate::from_ymd_opt(x.year(), x.month(), 1).unwrap(),
        Period::Week => {
            NaiveDate::from_isoywd_opt(x.iso_week().year(), x.iso_week().week(), Weekday::Mon)
                .unwrap()
        }
    }
}

pub fn eop(x: &NaiveDate, p: Period) -> NaiveDate {
    match p {
        Period::Year => NaiveDate::from_ymd_opt(x.year(), 12, 31).unwrap(),
        Period::Semiannual => match x.month() {
            1..=6 => NaiveDate::from_ymd_opt(x.year(), 6, 30).unwrap(),
            _ => NaiveDate::from_ymd_opt(x.year(), 12, 31).unwrap(),
        },
        Period::Quarter => match x.month() {
            1..=3 => NaiveDate::from_ymd_opt(x.year(), 3, 31).unwrap(),
            4..=6 => NaiveDate::from_ymd_opt(x.year(), 6, 30).unwrap(),
            7..=9 => NaiveDate::from_ymd_opt(x.year(), 9, 30).unwrap(),
            _ => NaiveDate::from_ymd_opt(x.year(), 12, 31).unwrap(),
        },
        Period::Month => {
            let bop = NaiveDate::from_ymd_opt(x.year(), x.month(), 1).unwrap();
            add_days(&add_months(&bop, 1), -1)
        }
        Period::Week => {
            NaiveDate::from_isoywd_opt(x.iso_week().year(), x.iso_week().week(), Weekday::Sun)
                .unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    #[test]
    fn test_add_days() {
        let fromymd = NaiveDate::from_ymd_opt;
        assert_eq!(
            add_days(&fromymd(2021, 1, 1).unwrap(), -1),
            fromymd(2020, 12, 31).unwrap()
        );
        assert_eq!(
            add_days(&fromymd(2021, 1, 31).unwrap(), 1),
            fromymd(2021, 2, 1).unwrap()
        );
        assert_eq!(
            add_days(&fromymd(2021, 12, 31).unwrap(), 1),
            fromymd(2022, 1, 1).unwrap()
        );
    }
    #[test]
    fn test_add_months() {
        let fromymd = NaiveDate::from_ymd_opt;
        assert_eq!(
            add_months(&fromymd(2021, 1, 1).unwrap(), -1),
            fromymd(2020, 12, 1).unwrap()
        );
        assert_eq!(
            add_months(&fromymd(2021, 1, 31).unwrap(), 1),
            fromymd(2021, 2, 28).unwrap()
        );
        assert_eq!(
            add_months(&fromymd(2021, 12, 31).unwrap(), 12),
            fromymd(2022, 12, 31).unwrap()
        );
    }
    #[test]
    fn test_bop() {
        let fromymd = NaiveDate::from_ymd_opt;
        assert_eq!(
            bop(&fromymd(2021, 1, 15).unwrap(), Period::Year),
            fromymd(2021, 1, 1).unwrap()
        );
        assert_eq!(
            bop(&fromymd(2021, 12, 15).unwrap(), Period::Semiannual),
            fromymd(2021, 7, 1).unwrap()
        );
        assert_eq!(
            bop(&fromymd(2021, 5, 15).unwrap(), Period::Quarter),
            fromymd(2021, 4, 1).unwrap()
        );
        assert_eq!(
            bop(&fromymd(2021, 8, 31).unwrap(), Period::Month),
            fromymd(2021, 8, 1).unwrap()
        );
        assert_eq!(
            bop(&fromymd(2022, 1, 1).unwrap(), Period::Week),
            fromymd(2021, 12, 27).unwrap()
        );
    }
    #[test]
    fn test_eop() {
        let fromymd = NaiveDate::from_ymd_opt;
        assert_eq!(
            eop(&fromymd(2021, 1, 15).unwrap(), Period::Year),
            fromymd(2021, 12, 31).unwrap()
        );
        assert_eq!(
            eop(&fromymd(2021, 1, 15).unwrap(), Period::Semiannual),
            fromymd(2021, 6, 30).unwrap()
        );
        assert_eq!(
            eop(&fromymd(2021, 5, 15).unwrap(), Period::Quarter),
            fromymd(2021, 6, 30).unwrap()
        );
        assert_eq!(
            eop(&fromymd(2021, 2, 12).unwrap(), Period::Month),
            fromymd(2021, 2, 28).unwrap()
        );
        assert_eq!(
            eop(&fromymd(2022, 1, 1).unwrap(), Period::Week),
            fromymd(2022, 1, 2).unwrap()
        );
    }
}
