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

make_fun!(mday, |date: &NaiveDate| -> i32 { date.day() as i32 });

make_fun!(yday, |date: &NaiveDate| -> i32 { date.ordinal() as i32 });

make_fun!(wday, |date: &NaiveDate| -> i32 { date.weekday() as i32 });

// yday wday week yearmon yearqtr
