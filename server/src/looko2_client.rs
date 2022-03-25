use chrono::{NaiveDateTime, TimeZone, Utc};
use chrono_tz::Europe::Warsaw;
use regex::Regex;

use models::AirQualityData;

use crate::models;

pub async fn latest_get_look_o2_body(station_id: &String) -> Option<String> {
    let url: String = format!("https://looko2.com/tracker2.php?search={}", station_id);
    let body = match reqwest::get(url).await {
        Ok(r) if r.status() == reqwest::StatusCode::OK => {
            Some(r.text().await.unwrap_or(String::new()))
        }
        _ => None,
    };
    body
}

fn capture<'a>(pattern: Regex, capture_name: &'a str, body: &'a str) -> Option<&'a str> {
    pattern.captures(&body).and_then(|c| {
        let value = c.name(capture_name).map(|x| x.as_str());
        value
    })
}

const CAPTURE_NAME_DATE: &str = "date";
const CAPTURE_NAME_PM25: &str = "pm25";

pub fn parse_look_o2_body(body: String) -> Option<AirQualityData> {
    let pm25_pattern = Regex::new(r#"<H4>PM2\.5</H4><BR>(?P<pm25>.*) ug"#).unwrap();
    let date_pattern = Regex::new(r#"</h6><h6>Ostatnio widziany: (?P<date>.*)</h6>"#).unwrap();
    if body.len() < 1000 {
        None
    } else {
        let pm25_capture = capture(pm25_pattern, CAPTURE_NAME_PM25, &body);
        let date_capture = capture(date_pattern, CAPTURE_NAME_DATE, &body);

        let pm25_value = pm25_capture.map(|c| c.parse::<u8>().unwrap_or(0));
        let date_value = date_capture.map(|c| c);

        match (pm25_value, date_value) {
            (Some(pm25), Some(date)) => {
                let date = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S").unwrap();
                let warsaw_time_in_utc = Warsaw
                    .from_local_datetime(&date)
                    .unwrap()
                    .with_timezone(&Utc);
                Some(AirQualityData {
                    pm25,
                    date: warsaw_time_in_utc,
                })
            }
            _ => None,
        }
    }
}
