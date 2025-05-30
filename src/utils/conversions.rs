use chrono::{
    Datelike, Local, TimeDelta, TimeZone
};
use anyhow::Result;

pub fn date_time_generator(week_num: &String) -> Result<(String, String), String> {
    let today = Local::now();
    let weeks = week_num.parse::<i64>().unwrap();
    let weeks_delta = TimeDelta::weeks(weeks);
    let target_date = today + weeks_delta;
    let year = target_date.year();
    let month = target_date.month();
    let day = target_date.month();

    let time_delineator = Local
        .with_ymd_and_hms(year, month, day, 20, 00, 00)
        .single().unwrap();

    let time_of_day = if target_date < time_delineator {
        "evening"
    } else {
        "night"
    };

    let time = today.format("%H:%M:%S").to_string();
    let target_date = target_date.format("%Y-%m-%d").to_string();

    Ok((time_of_day.to_string(), target_date))
}