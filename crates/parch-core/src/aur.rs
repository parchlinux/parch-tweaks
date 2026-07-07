use anyhow::Result;

#[derive(Debug, Clone)]
pub struct AurHelper {
    pub name: &'static str,
    pub package: &'static str,
    pub is_default: bool,
}

pub const AUR_HELPERS: &[AurHelper] = &[
    AurHelper {
        name: "paru",
        package: "paru",
        is_default: true,
    },
    AurHelper {
        name: "yay",
        package: "yay",
        is_default: false,
    },
];

pub fn get_installed_aur_helper() -> Option<String> {
    for helper in AUR_HELPERS {
        if crate::run_command("which", &[helper.name]).is_ok() {
            return Some(helper.name.to_string());
        }
    }
    None
}

pub async fn switch_aur_helper(target: &AurHelper) -> Result<()> {
    let installed = get_installed_aur_helper();
    if let Some(ref current) = installed {
        if current == target.name {
            return Ok(());
        }
        let _ = crate::run_pkexec(&["pacman", "-Rds", current, "--noconfirm"]).await;
    }
    crate::run_pkexec(&["pacman", "-Sy", target.package, "--noconfirm"]).await?;
    Ok(())
}
