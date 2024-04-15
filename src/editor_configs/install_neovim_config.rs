use crate::execute_command;

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

pub fn lazyvim() {
    execute_command("sudo", vec!["pacman", "-S", "neovim", "git", "--noconfirm"]);
    execute_command(
        "git",
        vec![
            "clone",
            "https://github.com/LazyVim/starter",
            "~/.config/nvim",
        ],
    );
    println!("new you can run neovim by typing: nvim");
}
