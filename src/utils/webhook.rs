#[derive(Debug, Clone, serde::Serialize)]
pub struct WebhookField {
    name: String,
    value: String,
    inline: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Webhook {
    pub name: String,
    pub url: String,
    pub title: String,
    pub content: String,
    pub footer: String,
    pub thumbnail: String,
    pub avatar: String,
    pub fields: Vec<WebhookField>,
}

pub async fn send(webhook: &Webhook) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let payload = serde_json::to_string(webhook)?;

    let response = client
        .post(&webhook.url)
        .header("Content-Type", "application/json")
        .body(payload)
        .send()
        .await?;

    Ok(())
}
