use anyhow::Result;

#[derive(Debug, Clone)]
pub struct EmojiSet {
    pub name: &'static str,
    pub package: &'static str,
    pub description: &'static str,
}

pub const EMOJI_SETS: &[EmojiSet] = &[
    EmojiSet {
        name: "iOS",
        package: "parch-emoji-ios",
        description: "Apple-style emoji for Parch Linux",
    },
    EmojiSet {
        name: "iOS (Arad)",
        package: "parch-emoji-ios-arad",
        description: "iOS emoji with Arad font variant",
    },
    EmojiSet {
        name: "iOS (Estedad)",
        package: "parch-emoji-ios-estedad",
        description: "iOS emoji with Estedad font variant",
    },
    EmojiSet {
        name: "iOS (Mikhak)",
        package: "parch-emoji-ios-mikhak",
        description: "iOS emoji with Mikhak font variant",
    },
    EmojiSet {
        name: "Fluent",
        package: "fluent-emojies",
        description: "Microsoft Fluent-style emoji",
    },
];

pub fn get_installed_emoji() -> Option<String> {
    for set in EMOJI_SETS {
        if crate::run_command("pacman", &["-Qi", set.package]).is_ok() {
            return Some(set.package.to_string());
        }
    }
    None
}

pub async fn install_emoji(package: &str) -> Result<()> {
    let installed = get_installed_emoji();
    if let Some(ref old) = installed {
        if old == package {
            return Ok(());
        }
        let _ = crate::run_pkexec(&["pacman", "-Rds", old, "--noconfirm"]).await;
    }
    crate::run_pkexec(&["pacman", "-Sy", package, "--noconfirm"]).await?;
    Ok(())
}
