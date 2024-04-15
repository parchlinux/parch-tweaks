mod install_neovim_config;
mod install_emacs_config;

use crate::{execute_command, get_choice, INVALID_CHOICE, PLEASE_CHOOSE};

pub fn install_editor_configs() {
    println!("{PLEASE_CHOOSE} your editor configs:\n\
        1) neovim\n\
        2) emacs\n\
        3) vim");

    let ediconf = get_choice();
    match ediconf.as_str() {
        "1" => {
            println!("{PLEASE_CHOOSE} your neovim config:\n\
                1) nvchad\n\
                2) nvpak\n\
                3) lunarvim\n\
                4) lazyvim");

            let neoconf = get_choice();
            match neoconf.as_str() {
                "1" => install_neovim_config::nvchad(),
                "2" => install_neovim_config::nvpack(),
                "3" => install_neovim_config::lunarvim(),
                "4" => install_neovim_config::lazyvim(),
                _ => println!("{INVALID_CHOICE}"),
            }
        }
        "2" => {
            println!("{PLEASE_CHOOSE} your editor configs:\n\
                1) DoomEmacs\n\
                2) Spacemacs");

            let emacconf = get_choice();
            match emacconf.as_str() {
                "1" => install_emacs_config::doomemacs(),
                "2" => install_emacs_config::spacemacs(),
                _ => println!("{INVALID_CHOICE}"),
            }
        }
        "3" => println!("Soon..."),
        _ => println!("{INVALID_CHOICE}"),
    }
}
