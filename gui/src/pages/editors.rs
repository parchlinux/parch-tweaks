use adw::prelude::*;
use crate::log_window;
use gtk;
use parch_core::editor;

pub fn build_page(toast_overlay: &adw::ToastOverlay) -> adw::NavigationPage {
    let page = adw::NavigationPage::builder()
        .title("Editors")
        .tag("editors")
        .build();

    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(12)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .build();

    let title = gtk::Label::builder()
        .label("<big>Editor Configurations</big>")
        .use_markup(true)
        .halign(gtk::Align::Start)
        .build();
    content.append(&title);

    let desc = gtk::Label::builder()
        .label("Install pre-configured editor setups")
        .halign(gtk::Align::Start)
        .build();
    desc.add_css_class("dim-label");
    content.append(&desc);

    let current = editor::get_installed_editor();
    let current_label = gtk::Label::builder()
        .label(&format!(
            "Current editor: {}",
            current.as_deref().unwrap_or("None detected")
        ))
        .halign(gtk::Align::Start)
        .build();
    content.append(&current_label);

    content.append(&gtk::Separator::new(gtk::Orientation::Horizontal));

    let installed_config = editor::get_installed_config();

    let neovim_group = adw::PreferencesGroup::builder()
        .title("Neovim Configurations")
        .build();

    for config in editor::NEOVIM_CONFIGS {
        let row = adw::ActionRow::builder()
            .title(config.name)
            .subtitle(config.repo)
            .activatable(false)
            .build();

        if installed_config.as_deref() == Some(config.name) {
            let check = gtk::Image::from_icon_name("object-select-symbolic");
            check.add_css_class("success");
            row.add_suffix(&check);
        } else {
            let install_btn = gtk::Button::builder()
                .label("Install")
                .valign(gtk::Align::Center)
                .css_classes(["suggested-action"])
                .build();

            row.add_suffix(&install_btn);

            let btn_ref = install_btn.clone();
            let row_ref = row.clone();
            let toast_clone = toast_overlay.clone();
            let current_label_clone = current_label.clone();
            let config_name = config.name.to_string();
            let config_repo = config.repo.to_string();

            install_btn.connect_clicked(move |_| {
                let parent = match btn_ref.root().and_downcast::<gtk::Window>() {
                    Some(w) => w,
                    None => return,
                };

                if let Err(e) = editor::check_deps(&["nvim", "git"]) {
                    toast_clone.add_toast(adw::Toast::new(&format!("{}", e)));
                    return;
                }

                let target = format!(
                    "{}/.config/nvim",
                    std::env::var("HOME").unwrap_or_else(|_| "/root".to_string())
                );

                let toast = toast_clone.clone();
                let label = current_label_clone.clone();
                let name = config_name.clone();
                let repo = config_repo.clone();
                let row_ref2 = row_ref.clone();
                let btn_ref2 = btn_ref.clone();

                let target2 = target.clone();

                log_window::run_commands(
                    &format!("Installing {}...", config_name),
                    &parent,
                    vec![
                        ("rm".to_string(), vec!["-rf".to_string(), target]),
                        ("git".to_string(), vec!["clone".to_string(), "--depth".to_string(), "1".to_string(), repo, target2]),
                    ],
                    move || {
                        btn_ref2.set_visible(false);
                        let check = gtk::Image::from_icon_name("object-select-symbolic");
                        check.add_css_class("success");
                        row_ref2.add_suffix(&check);
                        editor::save_installed_config(&name).ok();
                        toast.add_toast(adw::Toast::new(&format!("Installed {} config", name)));
                        label.set_text(&format!("Installed: {}", name));
                    },
                );
            });
        }

        neovim_group.add(&row);
    }

    content.append(&neovim_group);

    let emacs_group = adw::PreferencesGroup::builder()
        .title("Emacs Configurations")
        .build();

    for config in editor::EMACS_CONFIGS {
        let row = adw::ActionRow::builder()
            .title(config.name)
            .subtitle(config.repo)
            .activatable(false)
            .build();

        if installed_config.as_deref() == Some(config.name) {
            let check = gtk::Image::from_icon_name("object-select-symbolic");
            check.add_css_class("success");
            row.add_suffix(&check);
        } else {
            let install_btn = gtk::Button::builder()
                .label("Install")
                .valign(gtk::Align::Center)
                .css_classes(["suggested-action"])
                .build();

            row.add_suffix(&install_btn);

            let btn_ref = install_btn.clone();
            let row_ref = row.clone();
            let toast_clone = toast_overlay.clone();
            let current_label_clone = current_label.clone();
            let config_name = config.name.to_string();
            let config_repo = config.repo.to_string();

            install_btn.connect_clicked(move |_| {
                let parent = match btn_ref.root().and_downcast::<gtk::Window>() {
                    Some(w) => w,
                    None => return,
                };

                if let Err(e) = editor::check_deps(&["emacs", "git"]) {
                    toast_clone.add_toast(adw::Toast::new(&format!("{}", e)));
                    return;
                }

                let target = if config_name == "Spacemacs" {
                    format!(
                        "{}/.emacs.d",
                        std::env::var("HOME").unwrap_or_else(|_| "/root".to_string())
                    )
                } else {
                    format!(
                        "{}/.config/emacs",
                        std::env::var("HOME").unwrap_or_else(|_| "/root".to_string())
                    )
                };

                let toast = toast_clone.clone();
                let label = current_label_clone.clone();
                let name = config_name.clone();
                let repo = config_repo.clone();
                let row_ref2 = row_ref.clone();
                let btn_ref2 = btn_ref.clone();
                let target2 = target.clone();

                log_window::run_commands(
                    &format!("Installing {}...", config_name),
                    &parent,
                    vec![
                        ("rm".to_string(), vec!["-rf".to_string(), target]),
                        ("git".to_string(), vec!["clone".to_string(), "--depth".to_string(), "1".to_string(), repo, target2]),
                    ],
                    move || {
                        btn_ref2.set_visible(false);
                        let check = gtk::Image::from_icon_name("object-select-symbolic");
                        check.add_css_class("success");
                        row_ref2.add_suffix(&check);
                        editor::save_installed_config(&name).ok();
                        toast.add_toast(adw::Toast::new(&format!("Installed {} config", name)));
                        label.set_text(&format!("Installed: {}", name));
                    },
                );
            });
        }

        emacs_group.add(&row);
    }

    content.append(&emacs_group);
    page.set_child(Some(&content));
    page
}
