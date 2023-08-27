use chrono::{DateTime, Duration, NaiveDateTime, Utc};

use crate::constants::BASE_URL;

pub struct DateTime8601String;

impl DateTime8601String {
    fn format(datetime: NaiveDateTime) -> String {
        datetime.format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }

    pub fn to_datetime(datetime_string: &String) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(datetime_string)
            .unwrap_or_default()
            .with_timezone(&Utc)
    }

    pub fn to_human_readable(datetime_string: &String) -> String {
        DateTime8601String::to_datetime(datetime_string)
            .format("%d %m %Y %H:%M")
            .to_string()
    }

    pub fn now() -> String {
        let datetime = Utc::now().naive_utc();
        Self::format(datetime)
    }

    pub fn now_plus_ms(milliseconds: i64) -> String {
        let datetime = Utc::now().naive_utc();
        let duration = Duration::milliseconds(milliseconds);
        Self::format(datetime + duration)
    }

    pub fn is_past(datetime_string: &String) -> bool {
        // TODO why is this so fucking complicated
        let now: DateTime<Utc> = DateTime::from_utc(Utc::now().naive_utc(), Utc);
        if let Ok(t) = DateTime::parse_from_rfc3339(datetime_string) {
            now > t.with_timezone(&Utc)
        } else {
            // TODO throw a log here but return true to fail gracefully
            true
        }
    }
}

pub fn get_own_url_with(append: &str) -> String {
    format!("{}{}", BASE_URL, append)
}

pub static ALPHANUMERIC_UNDERSCORE_RX: lazy_regex::Lazy<lazy_regex::Regex> =
    lazy_regex::lazy_regex!("^[a-zA-Z_]+$");

pub static SIMPLE_EMAIL_RX: lazy_regex::Lazy<lazy_regex::Regex> =
    lazy_regex::lazy_regex!(r"^.+\@.+\..+$");
