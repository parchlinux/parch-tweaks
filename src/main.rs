use std::{
    io,
    io::Write,
    process::{self, Command},
};

const INVALID_CHOICE: &str = "Invalid choice.";
const PLEASE_CHOOSE: &str = "Please choose an action for";

fn execute_command(command: &str, args: Vec<&str>) {
    let output = Command::new(command)
        .args(args)
        .output()
        .expect("failed to execute the process");

    if output.status.success() {
        io::stdout().write_all(&output.stdout).unwrap();
    } else {
        io::stdout().write_all(&output.stderr).unwrap();
    }
}

fn get_choice() -> String {
    println!("Enter your choice: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap_or_else(|_| process::exit(1));

    input.trim().to_owned()
}

fn install_emoji_packages() {
    println!("{PLEASE_CHOOSE} emoji Packages:\n\
        1) ios\n\
        2) fluent (microsoft)");

    let emoji_choice = get_choice();
    match emoji_choice.as_str() {
        "1" => {
            execute_command(
                "sudo",
                vec!["pacman", "-Rds", "fluent-emojies", "--noconfirm"],
            );
            execute_command(
                "sudo",
                vec!["pacman", "-Sy", "parch-emoji-ios", "--noconfirm"],
            );
        }
        "2" => {
            execute_command(
                "sudo",
                vec!["pacman", "-Rds", "parch-emoji-ios", "--noconfirm"],
            );
            execute_command(
                "sudo",
                vec!["pacman", "-Sy", "fluent-emojies", "--noconfirm"],
            );
        }
        _ => println!("{INVALID_CHOICE}"),
    }
}

fn install_libreoffice() {
    execute_command(
        "sudo",
        vec!["pacman", "-Sy", "libreoffice-fresh", "--noconfirm"],
    );
    execute_command("rm", vec!["-rf", "~/.config/liberoffice"]);
    execute_command(
        "git",
        vec![
            "clone",
            "https://github.com/parchlinux/libreoffice-config",
            "/tmp/liberoffice",
        ],
    );
    execute_command(
        "cp",
        vec!["-r", "/tmp/liberoffice/liberoffice", "~/.config"],
    )
}

fn change_aur_helper() {
    println!("{PLEASE_CHOOSE} aur helper:\n\
        1) yay\n\
        2) paru (default in parchlinux");

    let aur = get_choice();
    match aur.as_str() {
        "1" => {
            execute_command("sudo", vec!["pacman", "-Rds", "paru", "--noconfirm"]);
            execute_command("sudo", vec!["pacman", "-Sy", "yay", "--noconfirm"]);
        }
        "2" => {
            execute_command("sudo", vec!["pacman", "-Rds", "yay", "--noconfirm"]);
            execute_command("sudo", vec!["pacman", "-Sy", "paru", "--noconfirm"]);
        }
        _ => println!("{INVALID_CHOICE}"),
    }
}

mod install_neovim_config {
    use super::*;

    pub fn nvchad() {
        execute_command("sudo", vec!["pacman", "-S", "neovim", "git", "--noconfirm"]);
        execute_command(
            "git",
            vec![
                "clone",
                "https://github.com/NvChad/NvChad",
                "~/.config/nvim",
                "--depth",
                "1",
            ],
        );
        println!("now you can run neovim by typing: nvim")
    }

    pub fn nvpack() {
        execute_command("sudo", vec!["pacman", "-S", "neovim", "git", "--noconfirm"]);
        execute_command(
            "git",
            vec![
                "clone",
                "--depth",
                "1",
                "https://github.com/Pakrohk-DotFiles/NvPak.git",
                " ~/.config/nvim",
            ],
        );
        println!("now you can run neovim by typing: nvim");
    }

    pub fn lunarvim() {
        execute_command("sudo", vec!["pacman", "-S", "neovim", "git", "--noconfirm"]);
        execute_command(
            "LV_BRANCH='release-1.3/neovim-0.9'",
            vec!["bash", "<(curl", "-s",
            "https://raw.githubusercontent.com/LunarVim/LunarVim/release-1.3/neovim-0.9/utils/installer/install.sh)"]
        );
    }
}

mod install_emacs_config {
    use super::*;

    pub fn doomemacs() {
        execute_command("sudo", vec!["pacman", "-S", "emacs", "git", "--noconfirm"]);
        execute_command(
            "git",
            vec![
                "clone",
                "--depth",
                "1",
                "https://github.com/doomemacs/doomemacs",
                "~/.config/emacs",
            ],
        );
        println!("dont forgot to add ~/.config/emacs/bin to your path.");
        println!("after adding to path, re run your terminal and type doom install.");
    }

    pub fn spacemacs() {
        execute_command("sudo", vec!["pacman", "-S", "emacs", "git", "--noconfirm"]);
        execute_command(
            "git",
            vec![
                "clone",
                "https://github.com/syl20bnr/spacemacs",
                "~/.emacs.d",
            ],
        );
        println!("now you can now open emacs for changes to take effect.");
    }
}

fn install_editor_configs() {
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
                3) lunarvim");

            let neoconf = get_choice();
            match neoconf.as_str() {
                "1" => install_neovim_config::nvchad(),
                "2" => install_neovim_config::nvpack(),
                "3" => install_neovim_config::lunarvim(),
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

fn main() {
    execute_command("figlet", vec!["Parch Linux tweak tool"]);

    println!("Please select a section:\n\
        1) Change Emoji package\n\
        2) Install Libreoffice\n\
        3) Change Aur Helper\n\
        4) Install Editor configs\n\
        5) Exit");

    let choice = get_choice();
    match choice.as_str() {
        "1" => install_emoji_packages(),
        "2" => install_libreoffice(),
        "3" => change_aur_helper(),
        "4" => install_editor_configs(),
        "5" => process::exit(0),
        _ => println!("{INVALID_CHOICE}"),
    }
    println!("done with section {}.", choice);
}
