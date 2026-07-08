use std::{
    io,
    process,
};

use parch_core::{aur, emoji, editor};

const INVALID_CHOICE: &str = "Invalid choice.";
const PLEASE_CHOOSE: &str = "Please choose an action for";

fn get_choice() -> String {
    println!("Enter your choice: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap_or_else(|_| process::exit(1));
    input.trim().to_owned()
}

fn install_emoji_packages() {
    let sets = emoji::EMOJI_SETS;
    for (i, set) in sets.iter().enumerate() {
        println!("  {}) {} — {}", i + 1, set.name, set.description);
    }

    let choice = get_choice();
    if let Ok(idx) = choice.parse::<usize>() {
        if idx >= 1 && idx <= sets.len() {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(emoji::install_emoji(sets[idx - 1].package))
                .expect("failed to install emoji");
            println!("Installed {} emoji set.", sets[idx - 1].name);
            return;
        }
    }
    println!("{INVALID_CHOICE}");
}

fn change_aur_helper() {
    let helpers = aur::AUR_HELPERS;
    for (i, helper) in helpers.iter().enumerate() {
        let default = if helper.is_default { " (default)" } else { "" };
        println!("  {}) {}{}", i + 1, helper.name, default);
    }

    let choice = get_choice();
    if let Ok(idx) = choice.parse::<usize>() {
        if idx >= 1 && idx <= helpers.len() {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(aur::switch_aur_helper(&helpers[idx - 1]))
                .expect("failed to switch AUR helper");
            println!("Switched to {}.", helpers[idx - 1].name);
            return;
        }
    }
    println!("{INVALID_CHOICE}");
}

fn install_editor_configs() {
    println!("{PLEASE_CHOOSE} your editor:\n  1) Neovim\n  2) Emacs\n  3) Vim (coming soon)");

    let choice = get_choice();
    match choice.as_str() {
        "1" => {
            let configs = editor::NEOVIM_CONFIGS;
            for (i, config) in configs.iter().enumerate() {
                println!("  {}) {}", i + 1, config.name);
            }
            let neo_choice = get_choice();
            if let Ok(idx) = neo_choice.parse::<usize>() {
                if idx >= 1 && idx <= configs.len() {
                    editor::check_deps(&["nvim", "git"]).expect("dependencies missing");
                    let config = &configs[idx - 1];
                    editor::install_neovim_config(config.name, config.repo)
                        .expect("failed to install neovim config");
                    println!("Installed {} config.", configs[idx - 1].name);
                }
            }
        }
        "2" => {
            let configs = editor::EMACS_CONFIGS;
            for (i, config) in configs.iter().enumerate() {
                println!("  {}) {}", i + 1, config.name);
            }
            let emacs_choice = get_choice();
            if let Ok(idx) = emacs_choice.parse::<usize>() {
                if idx >= 1 && idx <= configs.len() {
                    editor::check_deps(&["emacs", "git"]).expect("dependencies missing");
                    let config = &configs[idx - 1];
                    editor::install_emacs_config(config.name, config.repo)
                        .expect("failed to install emacs config");
                    println!("Installed {} config.", configs[idx - 1].name);
                }
            }
        }
        "3" => println!("Coming soon..."),
        _ => println!("{INVALID_CHOICE}"),
    }
}

fn main() {
    let _ = std::process::Command::new("figlet").arg("Parch Linux tweak tool").output();

    println!(
        "Please select a section:\n\
         1) Emoji\n\
         2) AUR Helper\n\
         3) Editor Configs\n\
         4) Exit"
    );

    let choice = get_choice();
    match choice.as_str() {
        "1" => install_emoji_packages(),
        "2" => change_aur_helper(),
        "3" => install_editor_configs(),
        "4" => process::exit(0),
        _ => println!("{INVALID_CHOICE}"),
    }
    println!("Done.");
}
