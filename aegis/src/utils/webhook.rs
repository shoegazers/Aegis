use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct WebhookField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct EmbedFooter {
    pub text: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct EmbedThumbnail {
    pub url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Embed {
    pub title: String,
    pub description: String,
    pub footer: EmbedFooter,
    pub thumbnail: EmbedThumbnail,
    pub fields: Vec<WebhookField>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WebhookPayload {
    pub username: String,
    pub avatar_url: String,
    pub content: String,
    pub embeds: Vec<Embed>,
}

pub async fn send(
    webhook_url: &str,
    payload: &WebhookPayload,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client.post(webhook_url).json(payload).send().await?;

    println!("status: {}", response.status());
    println!("body: {}", response.text().await?);

    Ok(())
}
