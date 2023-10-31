import os

while True:
    os.system("figlet 'Parch Linux tweak tool'")
    print("Please select a section:")
    print("1) Change Emoji package")
    print("2) Libreoffice Installer")
    print("3) Change Aur Helper")
    print("4) Install Editor configs")
    print("5) Exit")

    choice = input("Enter your choice: ")

    if choice == "1":
        print("Please choose an action for emoji Packages:")
        print("1) ios")
        print("2) fluent (microsoft)")
        emoji_choice = input("Enter your choice: ")

        if emoji_choice == "1":
            os.system("sudo pacman -Rds fluent-emojies --noconfirm")
            os.system("sudo pacman -Sy parch-emoji-ios --noconfirm")
        elif emoji_choice == "2":
            os.system("sudo pacman -Rds parch-emoji-ios --noconfirm")
            os.system("sudo pacman -Sy fluent-emojies --noconfirm")
        else:
            print("Invalid choice.")
    elif choice == "2":
        os.system("sudo pacman -Sy libreoffice-fresh --noconfirm")
        os.system("rm -rf ~/.config/libreoffice")
        os.system("git clone https://github.com/parchlinux/libreoffice-config /tmp/libreoffice")
        os.system("cp -r /tmp/libreoffice/libreoffice ~/.config")
    elif choice == "3":
        print("Please choose an action for aur helper:")
        print("1) yay")
        print("2) paru (default in parchlinux)")
        aur = input("Enter your choice: ")

        if aur == "1":
            os.system("sudo pacman -Rds paru --noconfirm")
            os.system("sudo pacman -Sy yay --noconfirm")
        elif aur == "2":
            os.system("sudo pacman -Rds yay --noconfirm")
            os.system("sudo pacman -Sy paru --noconfirm")
        else:
            print("Invalid choice.")
    elif choice == "4":
        print("Please choose an action for your editor configs:")
        print("1) neovim")
        print("2) emacs")
        print("3) vim")
        ediconf = input("Enter your choice: ")

        if ediconf == "1":
            print("Please choose an action for your editor configs:")
            print("1) nvchad")
            print("2) nvpak")
            print("3) lunarvim")
            neoconf = input("Enter your choice: ")

            if neoconf == "1":
                os.system("sudo pacman -S neovim git --noconfirm")
                os.system("git clone https://github.com/NvChad/NvChad ~/.config/nvim --depth 1")
                print("now you can run neovim by typing: nvim")
            elif neoconf == "2":
                os.system("sudo pacman -S neovim git --noconfirm")
                os.system("git clone --depth 1 https://github.com/Pakrohk-DotFiles/NvPak.git ~/.config/nvim")
                print("now you can run neovim by typing: nvim")
            elif neoconf == "3":
                os.system("sudo pacman -S neovim git --noconfirm")
                os.system("LV_BRANCH='release-1.3/neovim-0.9' bash <(curl -s https://raw.githubusercontent.com/LunarVim/LunarVim/release-1.3/neovim-0.9/utils/installer/install.sh)")
            else:
                print("Invalid choice.")
        elif ediconf == "2":
            print("You selected emacs. Add your emacs installation and configuration here.")
        elif ediconf == "3":
            print("You selected vim. Add your vim installation and configuration here.")
        else:
            print("Invalid choice.")
    elif choice == "5":
        exit(0)
    else:
        print("Invalid choice.")

    print(f"Done with section {choice}.")

