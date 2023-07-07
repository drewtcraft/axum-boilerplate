#[cfg(debug_assertions)]
pub const BASE_URL: &'static str = "http://localhost:8080";

#[cfg(not(debug_assertions))]
pub const BASE_URL: &'static str = "xxx";

#[cfg(debug_assertions)]
pub const SENDER_EMAIL: &'static str = "email.xxx.com";

#[cfg(not(debug_assertions))]
pub const SENDER_EMAIL: &'static str = "email.xxx";

pub const EMAILER_URL: &'static str = "https://api.sendgrid.com/v3/mail/send";
