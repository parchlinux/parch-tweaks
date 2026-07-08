use anyhow::Result;
use std::path::PathBuf;

fn state_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    PathBuf::from(home).join(".config/parch-tweaks")
}

pub fn save_installed_config(config_name: &str) -> Result<()> {
    let dir = state_dir();
    std::fs::create_dir_all(&dir)?;
    std::fs::write(dir.join("installed_config.txt"), config_name)?;
    Ok(())
}

pub fn get_installed_config() -> Option<String> {
    let path = state_dir().join("installed_config.txt");
    std::fs::read_to_string(path).ok().map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
}

#[derive(Debug, Clone)]
pub struct EditorConfig {
    pub name: &'static str,
    pub package: &'static str,
}

pub const EDITORS: &[EditorConfig] = &[
    EditorConfig {
        name: "Neovim",
        package: "neovim",
    },
    EditorConfig {
        name: "Emacs",
        package: "emacs",
    },
    EditorConfig {
        name: "Vim",
        package: "vim",
    },
];

#[derive(Debug, Clone)]
pub struct NeovimConfig {
    pub name: &'static str,
    pub repo: &'static str,
}

pub const NEOVIM_CONFIGS: &[NeovimConfig] = &[
    NeovimConfig {
        name: "NvChad",
        repo: "https://github.com/NvChad/NvChad",
    },
    NeovimConfig {
        name: "NvPak",
        repo: "https://github.com/Pakrohk-DotFiles/NvPak.git",
    },
    NeovimConfig {
        name: "LunarVim",
        repo: "https://github.com/LunarVim/LunarVim",
    },
    NeovimConfig {
        name: "LazyVim",
        repo: "https://github.com/LazyVim/starter",
    },
];

#[derive(Debug, Clone)]
pub struct EmacsConfig {
    pub name: &'static str,
    pub repo: &'static str,
}

pub const EMACS_CONFIGS: &[EmacsConfig] = &[
    EmacsConfig {
        name: "Doom Emacs",
        repo: "https://github.com/doomemacs/doomemacs",
    },
    EmacsConfig {
        name: "Spacemacs",
        repo: "https://github.com/syl20bnr/spacemacs",
    },
];

pub fn check_deps(programs: &[&str]) -> Result<()> {
    let mut missing = Vec::new();
    for &prog in programs {
        if crate::run_command("which", &[prog]).is_err() {
            missing.push(prog);
        }
    }

    if missing.is_empty() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Missing required programs: {}. Please install them with your package manager.",
            missing.join(", ")
        ))
    }
}

pub fn install_neovim_config(_name: &str, repo: &str) -> Result<()> {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    let target = format!("{}/.config/nvim", home);

    let _ = std::fs::remove_dir_all(&target);

    crate::run_command("git", &[
        "clone", "--depth", "1", repo, &target,
    ])?;

    Ok(())
}

pub fn install_emacs_config(name: &str, repo: &str) -> Result<()> {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    let target = if name == "Spacemacs" {
        format!("{}/.emacs.d", home)
    } else {
        format!("{}/.config/emacs", home)
    };

    let _ = std::fs::remove_dir_all(&target);

    crate::run_command("git", &[
        "clone", "--depth", "1", repo, &target,
    ])?;

    Ok(())
}

pub fn get_installed_editor() -> Option<String> {
    for editor in EDITORS {
        if crate::run_command("which", &[editor.name.to_lowercase().as_str()]).is_ok() {
            return Some(editor.name.to_string());
        }
    }
    None
}
