use crate::execute_command;

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
