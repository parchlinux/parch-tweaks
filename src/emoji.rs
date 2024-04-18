use std::{
    io,
    process::{self, Command},
};

use crate::{execute_command, get_choice, INVALID_CHOICE, PLEASE_CHOOSE};

pub fn install_emoji_packages() {
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
