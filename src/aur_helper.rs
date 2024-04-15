use crate::{execute_command, get_choice, INVALID_CHOICE, PLEASE_CHOOSE};

pub fn change_aur_helper() {
    println!("{PLEASE_CHOOSE} aur helper:\n\
        1) yay\n\
        2) paru (default in parchlinux)");

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
