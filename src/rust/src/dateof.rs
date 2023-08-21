use chrono::{Datelike, NaiveDate};

fn to_quarter(month: u32) -> i32 {
    match month {
        1..=3 => 1,
        4..=6 => 2,
        7..=9 => 3,
        10..=12 => 4,
        _ => {
            panic!("month must be 1..=12");
        }
    }
}

macro_rules! make_fun {
    ($fn_name:ident, $method:expr) => {
        pub fn $fn_name(x: &[Option<NaiveDate>]) -> Vec<Option<i32>> {
            x.iter()
                .map(|date: &Option<NaiveDate>| match date {
                    Some(date) => Some($method(date)),
                    None => None,
                })
                .collect()
        }
    };
}

make_fun!(year, |date: &NaiveDate| -> i32 { date.year() as i32 });

make_fun!(month, |date: &NaiveDate| -> i32 { date.month() as i32 });

make_fun!(quarter, |date: &NaiveDate| -> i32 {
    to_quarter(date.month())
});

make_fun!(isoweek, |date: &NaiveDate| -> i32 {
    date.iso_week().week() as i32
});

make_fun!(wday, |date: &NaiveDate| -> i32 {
    date.weekday().number_from_sunday() as i32
});

make_fun!(mday, |date: &NaiveDate| -> i32 { date.day() as i32 });

make_fun!(yday, |date: &NaiveDate| -> i32 { date.ordinal() as i32 });

make_fun!(isowday, |date: &NaiveDate| -> i32 {
    date.weekday().number_from_monday() as i32
});

// yday wday week yearmon yearqtr

#[cfg(test)]
mod test {
    use super::*;
    use chrono::NaiveDate;
    #[test]
    fn wday_works() {
        let date0 = NaiveDate::from_ymd_opt(2022, 1, 17).unwrap();
        let dates: Vec<Option<NaiveDate>> = (0..=6)
            .map(|i| {
                let date =
                    NaiveDate::from_num_days_from_ce_opt(date0.num_days_from_ce() + i).unwrap();
                Some(date)
            })
            .collect();
        let isowday_expect = vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
        ];
        assert_eq!(isowday(&dates), isowday_expect);
        let wday_expect = vec![
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(1),
        ];
        assert_eq!(wday(&dates), wday_expect);
    }
}
