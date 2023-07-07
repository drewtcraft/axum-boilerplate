use crate::constants::{EMAILER_URL, SENDER_EMAIL};
use crate::error::{Error, Result};

use std::env;

use reqwest::Client;
use serde_json::json;

pub async fn send_email(to: &str, subject: &str, html: String, plain_text: String) -> Result<bool> {
    let sendgrid_api_key = env::var("SENDGRID_API_KEY").expect("no sendgrid api key");
    Client::new()
        .post(EMAILER_URL)
        .header("Authorization", format!("Bearer {}", sendgrid_api_key))
        .header("Content-Type", "application/json")
        .body(
            json!({
                "personalizations": [{
                    "to": [{"email": to}]
                }],
                "from": {"email": SENDER_EMAIL},
                "subject": subject,
                "content": [
                    {
                        "type": "text/html",
                        "value": html,
                    }, {
                        "type": "text/plain",
                        "value": plain_text,
                    }],
            })
            .to_string(),
        )
        .send()
        .await
        .map(|res| res.status().is_success())
        .map_err(|_| Error::DatabaseFailure)
}
