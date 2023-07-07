use crate::error::{Error, Result};
use askama::Template;

#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseTemplate {
    pub content: String,
}

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate<'a> {
    pub error_message: &'a str,
    pub status_code: &'a str,
}

impl<'a> ErrorTemplate<'a> {
    pub fn new(error_message: &'a str, status_code: &'a str) -> String {
        Self { error_message, status_code }
            .render()
            .unwrap_or("Error template failure.".to_string())
    }
}
