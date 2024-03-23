use std::{
    io,
    process::{self, Command},
};

const INVALID_CHOICE: &str = "Invalid choice.";
const PLEASE_CHOOSE: &str = "Please choose an action for";

fn execute_command(command: &str, args: Vec<&str>) {
    Command::new(command)
        .args(args)
        .output()
        .expect("failed to execute the process");
}

fn get_choice() -> String {
    println!("Enter your choice: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap_or_else(|_| process::exit(1));

    input.trim().to_owned()
}

fn main() {
    execute_command("figlet", vec!["Parch Linux tweak tool"]);

    println!("Please select a section:");
    println!("1) Change Emoji package");
    println!("2) Libreoffice Installer");
    println!("3) Change Aur Helper");
    println!("4) Install Editor configs");
    println!("5) Exit");

    let choice = get_choice();
    match choice.as_str() {
        "1" => {
            println!("{PLEASE_CHOOSE} emoji Packages:");
            println!("1) ios");
            println!("2) fluent (microsoft)");
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
                &_ => println!("{INVALID_CHOICE}"),
            }
        }
        "2" => {
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
        "3" => {
            println!("{PLEASE_CHOOSE} aur helper:");
            println!("1) yay");
            println!("2) paru (default in parchlinux)");
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
                &_ => println!("{INVALID_CHOICE}"),
            }
        }
        "4" => {
            println!("{PLEASE_CHOOSE} your editor configs:");
            println!("1) neovim");
            println!("2) emacs");
            println!("3) vim");
            let ediconf = get_choice();
            match ediconf.as_str() {
                "1" => {
                    println!("{PLEASE_CHOOSE} your neovim config:");
                    println!("1) nvchad");
                    println!("2) nvpak");
                    println!("3) lunarvim");
                    let neoconf = get_choice();
                    match neoconf.as_str() {
                        "1" => {
                            execute_command(
                                "sudo",
                                vec!["pacman", "-S", "neovim", "git", "--noconfirm"],
                            );
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
                        "2" => {
                            execute_command(
                                "sudo",
                                vec!["pacman", "-S", "neovim", "git", "--noconfirm"],
                            );
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
                        "3" => {
                            execute_command(
                                "sudo",
                                vec!["pacman", "-S", "neovim", "git", "--noconfirm"],
                            );
                            execute_command("LV_BRANCH='release-1.3/neovim-0.9'", vec!["bash","<(curl","-s","https://raw.githubusercontent.com/LunarVim/LunarVim/release-1.3/neovim-0.9/utils/installer/install.sh)"]);
                        }
                        &_ => println!("{INVALID_CHOICE}"),
                    }
                }
                "2" => {
                    println!("{PLEASE_CHOOSE} your editor configs:");
                    println!("1) Dooedm emacs");
                    println!("2) Spaedcemacs");
                    let emacconf = get_choice();
                    match emacconf.as_str() {
                        "1" => {
                            execute_command(
                                "sudo",
                                vec!["pacman", "-S", "emacs", "git", "--noconfirm"],
                            );
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
                            println!(
                                "after adding to path, re run your terminal and type doom install."
                            );
                        }
                        "2" => {
                            execute_command(
                                "sudo",
                                vec!["pacman", "-S", "emacs", "git", "--noconfirm"],
                            );
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
                        "3" => {
                            println!("soon...");
                        }
                        &_ => println!("{INVALID_CHOICE}"),
                    }
                }
                &_ => println!("{INVALID_CHOICE}"),
            }
        }
        "5" => process::exit(0),
        &_ => {
            println!("{INVALID_CHOICE}")
        }
    }
    println!("done with section {}.", choice);
}
