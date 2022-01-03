use chrono::{Datelike, NaiveDate, Weekday};

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

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::{NaiveDate};
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
