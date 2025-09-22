use std::{fs::File, io::Write};
use reqwest::Client;
use scraper::{Html, Selector};

use anyhow::Result;
use chrono::{Days, NaiveDate};
use shared::definitions::Semester;

use crate::{get_semester_data, subjects_entries::get_mondays_from_to};

pub async fn get_and_write(semesters: &Vec<Semester>) -> Result<()> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (compatible; rust-reqwest-scraper/1.0)")
        .build()?;

    let mut work_free_days = vec![];

    for semester in semesters {
        println!("Semester: {semester:?}");
        let (start_date, end_date) = get_semester_data(semester);
        let mondays = get_mondays_from_to(start_date, end_date)?;

        for day in &mondays {
            let url = format!("https://urnik.fs.uni-lj.si/layer_one/82/?day={day}");
            println!("Scraping: {url}");

            let resp = client.get(&url).send().await?.text().await?;
            let document = Html::parse_document(&resp);

            let selector = Selector::parse(".day").unwrap();

            for (i, element) in document.select(&selector).enumerate() {
                let html_str = element.html();
                if html_str.contains("day-work-free") {
                    if let Some(d) = day.checked_add_days(Days::new(i as u64)) {
                        work_free_days.push(d);
                    }
                }
            }
        }
    }

    // save results just like before
    write_work_free_days_json("static/work_free_days.json", &work_free_days)?;

    Ok(())
}

pub fn write_work_free_days_json(file_path: &str, work_free_days: &Vec<NaiveDate>) -> std::io::Result<()> {
    // Serialize to pretty JSON
    let json_string = serde_json::to_string_pretty(work_free_days)
        .expect("Failed to serialize work_free_days to JSON");

    // Write JSON to file
    let mut file = File::create(file_path)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}
