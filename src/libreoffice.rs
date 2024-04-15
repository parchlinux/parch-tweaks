use crate::{execute_command, INVALID_CHOICE};

pub fn install_libreoffice() {
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
    );
}
