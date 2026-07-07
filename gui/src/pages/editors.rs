use adw::prelude::*;
use gtk::{self, glib};
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

    let neovim_group = adw::PreferencesGroup::builder()
        .title("Neovim Configurations")
        .build();

    for config in editor::NEOVIM_CONFIGS {
        let row = adw::ActionRow::builder()
            .title(config.name)
            .subtitle(config.repo)
            .activatable(false)
            .build();

        let install_btn = gtk::Button::builder()
            .label("Install")
            .valign(gtk::Align::Center)
            .css_classes(["suggested-action"])
            .build();

        row.add_suffix(&install_btn);

        let config_name = config.name.to_string();
        let config_repo = config.repo.to_string();
        let toast_clone = toast_overlay.clone();
        let current_label_clone = current_label.clone();

        install_btn.connect_clicked(move |_| {
            let toast_overlay = toast_clone.clone();
            let config_name = config_name.clone();
            let config_repo = config_repo.clone();
            let current_label = current_label_clone.clone();

            glib::spawn_future_local(async move {
                match editor::install_neovim_config(&config_name, &config_repo).await {
                    Ok(()) => {
                        let toast = adw::Toast::new(&format!("Installed {} config", config_name));
                        toast_overlay.add_toast(toast);
                        current_label.set_text(&format!("Installed: {}", config_name));
                    }
                    Err(e) => {
                        let toast = adw::Toast::new(&format!("Failed: {}", e));
                        toast_overlay.add_toast(toast);
                    }
                }
            });
        });

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

        let install_btn = gtk::Button::builder()
            .label("Install")
            .valign(gtk::Align::Center)
            .css_classes(["suggested-action"])
            .build();

        row.add_suffix(&install_btn);

        let config_name = config.name.to_string();
        let config_repo = config.repo.to_string();
        let toast_clone = toast_overlay.clone();

        install_btn.connect_clicked(move |_| {
            let toast_overlay = toast_clone.clone();
            let config_name = config_name.clone();
            let config_repo = config_repo.clone();

            glib::spawn_future_local(async move {
                match editor::install_emacs_config(&config_name, &config_repo).await {
                    Ok(()) => {
                        let toast = adw::Toast::new(&format!("Installed {} config", config_name));
                        toast_overlay.add_toast(toast);
                    }
                    Err(e) => {
                        let toast = adw::Toast::new(&format!("Failed: {}", e));
                        toast_overlay.add_toast(toast);
                    }
                }
            });
        });

        emacs_group.add(&row);
    }

    content.append(&emacs_group);
    page.set_child(Some(&content));
    page
}
