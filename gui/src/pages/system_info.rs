use adw::prelude::*;
use gtk::{self, glib};
use parch_core::system;
use std::process::Command;

fn is_installed(binary: &str) -> bool {
    Command::new("which")
        .arg(binary)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn build_page() -> adw::NavigationPage {
    let page = adw::NavigationPage::builder()
        .title("System Info")
        .tag("system_info")
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
        .label("<big>System Information</big>")
        .use_markup(true)
        .halign(gtk::Align::Start)
        .build();
    content.append(&title);

    let desc = gtk::Label::builder()
        .label("System details and quick links")
        .halign(gtk::Align::Start)
        .build();
    desc.add_css_class("dim-label");
    content.append(&desc);

    content.append(&gtk::Separator::new(gtk::Orientation::Horizontal));

    let info = system::get_system_info();

    let group = adw::PreferencesGroup::builder()
        .title("System Details")
        .build();

    let rows = [
        ("Operating System", info.os_name),
        ("Version", info.os_version),
        ("Kernel", info.kernel),
        ("Shell", info.shell),
        ("Emoji Set", info.emoji_set.unwrap_or_else(|| "None".to_string())),
        ("AUR Helper", info.aur_helper.unwrap_or_else(|| "None".to_string())),
    ];

    for (label, value) in &rows {
        let row = adw::ActionRow::builder()
            .title(*label)
            .subtitle(value.as_str())
            .activatable(false)
            .build();
        group.add(&row);
    }

    content.append(&group);

    let links_group = adw::PreferencesGroup::builder()
        .title("Related Tools")
        .build();

    let tools = [
        ("Driver Manager", "parch-driver-manager", "parch-driver-manager"),
        ("Kernel Manager", "parch-kernel-manager", "parch-kernel-manager"),
        ("Mirror Manager", "mirrorman", "mirrorman"),
        ("Graphical Store", "shelly-ui", "Shelly"),
    ];

    for (name, binary, display_name) in &tools {
        let installed = is_installed(binary);

        let row = if installed {
            adw::ActionRow::builder()
                .title(*name)
                .subtitle(format!("Launch {}", display_name))
                .activatable(true)
                .build()
        } else {
            adw::ActionRow::builder()
                .title(*name)
                .subtitle(format!("{} not found — install it with pacman", display_name))
                .activatable(false)
                .build()
        };

        if installed {
            let arrow = gtk::Image::from_icon_name("go-next-symbolic");
            row.add_suffix(&arrow);

            let cmd = binary.to_string();
            row.connect_activated(move |_| {
                let cmd = cmd.clone();
                let _ = glib::spawn_future_local(async move {
                    let _ = tokio::task::spawn_blocking(move || {
                        let _ = Command::new(&cmd).spawn();
                    })
                    .await;
                });
            });
        } else {
            let warning = gtk::Image::from_icon_name("dialog-warning-symbolic");
            warning.add_css_class("warning");
            row.add_suffix(&warning);
        }

        links_group.add(&row);
    }

    content.append(&links_group);
    page.set_child(Some(&content));
    page
}
