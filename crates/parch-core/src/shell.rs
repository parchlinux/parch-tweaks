use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ShellConfig {
    pub name: &'static str,
    pub description: &'static str,
    pub install_cmd: &'static str,
}

pub const SHELL_CONFIGS: &[ShellConfig] = &[
    ShellConfig {
        name: "Zsh + Oh My Zsh",
        description: "Zsh with Oh My Zsh framework and P10k prompt",
        install_cmd: "sh -c \"$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)\"",
    },
    ShellConfig {
        name: "Zsh + Zinit",
        description: "Zsh with Zinit plugin manager",
        install_cmd: "sh -c \"$(curl -fsSL https://raw.githubusercontent.com/zdharma-continuum/zinit/main/scripts/install.sh)\"",
    },
    ShellConfig {
        name: "Fish + Fisher",
        description: "Fish shell with Fisher plugin manager",
        install_cmd: "curl -sL https://raw.githubusercontent.com/jorgebucaran/fisher/main/functions/fisher.fish | source && fisher install jorgebucaran/fisher",
    },
];

pub fn get_current_shell() -> Option<String> {
    std::env::var("SHELL").ok()
}

pub async fn install_shell_config(install_cmd: &str) -> Result<()> {
    let _ = crate::run_pkexec(&["pacman", "-Sy", "zsh", "--noconfirm"]).await;
    crate::run_command("bash", &["-c", install_cmd])?;
    Ok(())
}
