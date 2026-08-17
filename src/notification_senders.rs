use reqwest::Client;
use serde_json::json;

use crate::{NTFY_API_KEY, NTFY_URL, errors::Result};

pub async fn send_notification(title: &str, message: &str, view_url: Option<String>) -> Result<()> {
    let client = Client::new();
    let mut body = json!({
        "topic": "look_up",
        "message": message,
        "title": title
    });

    if let Some(url) = view_url {
        body.as_object_mut().unwrap().insert(
            "actions".to_string(),
            json!([
                {
                    "action": "view",
                    "label": "Open Url",
                    "url": url
                }
            ]),
        );
    }
    let _ = client
        .post((*NTFY_URL).to_string())
        .bearer_auth(&*NTFY_API_KEY)
        .body(body.to_string())
        .send()
        .await?;

    Ok(())
}
