use std::{
    io,
    io::Write,
    process::{self, Command},
};

mod emoji;
mod libreoffice;
mod aur_helper;
mod editor_configs;

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
    emoji::install_emoji_packages();
}

fn install_libreoffice() {
    libreoffice::install_libreoffice();
}

fn change_aur_helper() {
    aur_helper::change_aur_helper();
}

fn install_editor_configs() {
    editor_configs::install_editor_configs();
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