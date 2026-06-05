use std::{env, fmt::format};

use reqwest::Client;
use serde_json::json;

use crate::{HASSIO_API_KEY, HASSIO_URL, errors::Result, util::Config};

pub async fn send_notification(config: &Config, title: &str, message: &str) -> Result<()> {
    let endpoint = format!("/api/services/notify/{}", config.notify_entity);
    let url = format!("{}{}", &*HASSIO_URL, endpoint);
    let hassio_data = json!({
        "message":message,
        "title":title,
    })
    .to_string();
    let client = Client::new();
    let req = client
        .post(&url)
        .bearer_auth(&*HASSIO_API_KEY)
        .header("Content-type", "application/json")
        .body(hassio_data)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}
