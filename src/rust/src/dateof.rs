use chrono::{Datelike, NaiveDate, Weekday};

pub fn year_frac(d1: &NaiveDate, d0: &NaiveDate) -> f64 {
  (d1.year() - d0.year()) as f64
  // must be as f64 first, otherwise u32 - u32 may overflow (when negative)
      + (d1.month() as f64 - d0.month() as f64) / 12.0
      + (d1.day() as f64 - d0.day() as f64) / 365.0
}

pub fn year(x: &[Option<NaiveDate>]) -> Vec<Option<i32>> {
  x.iter().map(|date: &Option<NaiveDate>| {
    match date {
      Some(date) => Some(date.year()),
      None => None,
    }
  }).collect()
}

pub fn month(x: &[Option<NaiveDate>]) -> Vec<Option<i32>> {
  x.iter().map(|date: &Option<NaiveDate>| {
    match date {
      Some(date) => Some(date.month() as i32),
      None => None,
    }
  }).collect()
}

fn month2quarter(x: u32) -> i32 {
  match x {
    1..=3 => 1,
    4..=6 => 2,
    7..=9 => 3,
    10..=12 => 4,
    _ => {
      panic!("x must be 1..=12");
    }
  }
}

pub fn quarter(x: &[Option<NaiveDate>]) -> Vec<Option<i32>> {
  x.iter().map(|date: &Option<NaiveDate>| {
    match date {
      Some(date) => Some(month2quarter(date.month())),
      None => None,
    }
  }).collect()
}

pub fn mday(x: &[Option<NaiveDate>]) -> Vec<Option<i32>> {
  x.iter().map(|date: &Option<NaiveDate>| {
    match date {
      Some(date) => Some(date.day() as i32),
      None => None,
    }
  }).collect()
}
