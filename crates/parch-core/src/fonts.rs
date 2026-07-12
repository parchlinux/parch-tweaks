use anyhow::Result;

#[derive(Debug, Clone)]
pub struct FontPackage {
    pub name: &'static str,
    pub package: &'static str,
    pub category: FontCategory,
    pub description: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FontCategory {
    Ui,
    Mono,
    Nerd,
    Other,
}

pub const FONT_PACKAGES: &[FontPackage] = &[
    FontPackage {
        name: "Inter",
        package: "inter-font",
        category: FontCategory::Ui,
        description: "Modern UI font",
    },
    FontPackage {
        name: "JetBrains Mono",
        package: "ttf-jetbrains-mono",
        category: FontCategory::Mono,
        description: "Programming font by JetBrains",
    },
    FontPackage {
        name: "Fira Code",
        package: "ttf-fira-code",
        category: FontCategory::Mono,
        description: "Monospaced font with ligatures",
    },
    FontPackage {
        name: "Nerd Fonts (FiraCode)",
        package: "ttf-firacode-nerd",
        category: FontCategory::Nerd,
        description: "FiraCode with Nerd Font icons",
    },
    FontPackage {
        name: "Nerd Fonts (JetBrains)",
        package: "ttf-jetbrains-mono-nerd",
        category: FontCategory::Nerd,
        description: "JetBrains Mono with Nerd Font icons",
    },
    FontPackage {
        name: "Noto Sans",
        package: "noto-fonts",
        category: FontCategory::Ui,
        description: "Google Noto font family",
    },
    FontPackage {
        name: "Cascadia Code",
        package: "ttf-cascadia-code",
        category: FontCategory::Mono,
        description: "Microsoft programming font",
    },
];

pub fn is_installed(package: &str) -> bool {
    crate::run_command("pacman", &["-Qi", package]).is_ok()
}

pub async fn install_font(package: &str) -> Result<()> {
    crate::run_pkexec(&["pacman", "-Sy", package, "--noconfirm"]).await?;
    Ok(())
}

pub fn get_category_label(cat: &FontCategory) -> &'static str {
    match cat {
        FontCategory::Ui => "UI Fonts",
        FontCategory::Mono => "Monospace Fonts",
        FontCategory::Nerd => "Nerd Fonts",
        FontCategory::Other => "Other",
    }
}
