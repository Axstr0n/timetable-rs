use chrono::{Datelike, NaiveDate};
use js_sys::Date;

pub fn current_iso_week() -> (i32, i32) {
    let now = Date::new_0();
    let year = now.get_full_year() as i32;
    let month = now.get_month() + 1;
    let day = now.get_date();

    let today = NaiveDate::from_ymd_opt(year, month, day).expect("valid date from JS");
    let iso = today.iso_week();
    (iso.year(), iso.week() as i32)
}

pub fn weeks_in_year(year: i32) -> i32 {
    NaiveDate::from_ymd_opt(year, 12, 28).unwrap().iso_week().week() as i32
}

