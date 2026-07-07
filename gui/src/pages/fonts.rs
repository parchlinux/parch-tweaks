use adw::prelude::*;
use gtk::{self, glib};
use parch_core::fonts::{self, FontCategory, FONT_PACKAGES, get_category_label};

pub fn build_page(toast_overlay: &adw::ToastOverlay) -> adw::NavigationPage {
    let page = adw::NavigationPage::builder()
        .title("Fonts")
        .tag("fonts")
        .build();

    let scrolled = gtk::ScrolledWindow::builder()
        .vexpand(true)
        .hscrollbar_policy(gtk::PolicyType::Never)
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
        .label("<big>Font Manager</big>")
        .use_markup(true)
        .halign(gtk::Align::Start)
        .build();
    content.append(&title);

    let desc = gtk::Label::builder()
        .label("Install and manage fonts for Parch Linux")
        .halign(gtk::Align::Start)
        .build();
    desc.add_css_class("dim-label");
    content.append(&desc);

    content.append(&gtk::Separator::new(gtk::Orientation::Horizontal));

    let categories: Vec<FontCategory> = vec![
        FontCategory::Ui,
        FontCategory::Mono,
        FontCategory::Nerd,
    ];

    for category in &categories {
        let group = adw::PreferencesGroup::builder()
            .title(get_category_label(category))
            .build();

        for font in FONT_PACKAGES {
            if &font.category != category {
                continue;
            }

            let installed = fonts::is_installed(font.package);

            let row = adw::ActionRow::builder()
                .title(font.name)
                .subtitle(font.description)
                .activatable(false)
                .build();

            if installed {
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

                let font_package = font.package.to_string();
                let font_name = font.name.to_string();
                let toast_clone = toast_overlay.clone();

                install_btn.connect_clicked(move |_| {
                    let toast_overlay = toast_clone.clone();
                    let font_package = font_package.clone();
                    let font_name = font_name.clone();

                    glib::spawn_future_local(async move {
                        match fonts::install_font(&font_package).await {
                            Ok(()) => {
                                let toast = adw::Toast::new(&format!("Installed {}", font_name));
                                toast_overlay.add_toast(toast);
                            }
                            Err(e) => {
                                let toast = adw::Toast::new(&format!("Failed: {}", e));
                                toast_overlay.add_toast(toast);
                            }
                        }
                    });
                });
            }

            group.add(&row);
        }

        content.append(&group);
    }

    scrolled.set_child(Some(&content));
    page.set_child(Some(&scrolled));
    page
}
