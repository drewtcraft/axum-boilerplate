use askama::Template;
use chrono::{DateTime, Duration, NaiveDateTime, Utc};

use crate::{
    error::{Error, Result},
    templates::BaseTemplate,
};

pub struct DateTime8601String;

impl DateTime8601String {
    fn format(datetime: NaiveDateTime) -> String {
        datetime.format("%Y-%m-%dT%H:%M:%SZ").to_string()
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

pub fn render(is_htmx: Option<bool>, partial: String) -> Result<String> {
    // should only really be needed on GET requests that could be
    // full page or a partial
    if let Some(is_htmx) = is_htmx {
        if !is_htmx {
            BaseTemplate { content: partial }
                .render()
                .map_err(|_| Error::TemplateRenderingFailure)
        } else {
            Ok(partial)
        }
    } else {
        Ok(partial)
    }
}
