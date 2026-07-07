use adw::prelude::*;
use gtk::{self, glib};
use parch_core::emoji;

pub fn build_page(toast_overlay: &adw::ToastOverlay) -> adw::NavigationPage {
    let page = adw::NavigationPage::builder()
        .title("Emoji")
        .tag("emoji")
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
        .label("<big>Emoji Configuration</big>")
        .use_markup(true)
        .halign(gtk::Align::Start)
        .build();
    content.append(&title);

    let desc = gtk::Label::builder()
        .label("Choose your preferred emoji set for Parch Linux")
        .halign(gtk::Align::Start)
        .build();
    desc.add_css_class("dim-label");
    content.append(&desc);

    let current = emoji::get_installed_emoji();
    let current_label = gtk::Label::builder()
        .label(&format!(
            "Current: {}",
            current.as_deref().unwrap_or("None installed")
        ))
        .halign(gtk::Align::Start)
        .build();
    content.append(&current_label);

    content.append(&gtk::Separator::new(gtk::Orientation::Horizontal));

    let group = adw::PreferencesGroup::builder()
        .title("Available Emoji Sets")
        .build();

    let selected = current.clone();
    let selected_ref = std::cell::RefCell::new(selected);

    for emoji_set in emoji::EMOJI_SETS {
        let row = adw::ActionRow::builder()
            .title(emoji_set.name)
            .subtitle(emoji_set.description)
            .activatable(false)
            .build();

        let radio = gtk::CheckButton::builder()
            .active(current.as_deref() == Some(emoji_set.package))
            .valign(gtk::Align::Center)
            .build();

        row.add_suffix(&radio);

        let package = emoji_set.package.to_string();
        let selected_clone = selected_ref.clone();

        radio.connect_toggled(move |is_active| {
            if is_active.is_active() {
                *selected_clone.borrow_mut() = Some(package.clone());
            }
        });

        group.add(&row);
    }

    content.append(&group);

    content.append(&gtk::Separator::new(gtk::Orientation::Horizontal));

    let apply_btn = gtk::Button::builder()
        .label("Apply Emoji Set")
        .css_classes(["suggested-action", "pill"])
        .halign(gtk::Align::End)
        .margin_top(12)
        .build();

    let toast_clone = toast_overlay.clone();
    let current_label_clone = current_label.clone();
    let selected_clone = selected_ref.clone();

    apply_btn.connect_clicked(move |_| {
        let toast_overlay = toast_clone.clone();
        let current_label = current_label_clone.clone();
        let selected = selected_clone.borrow().clone();

        if let Some(package) = selected {
            glib::spawn_future_local(async move {
                match emoji::install_emoji(&package).await {
                    Ok(()) => {
                        let toast = adw::Toast::new(&format!("Installed {} emoji set", package));
                        toast_overlay.add_toast(toast);
                        current_label.set_text(&format!("Current: {}", package));
                    }
                    Err(e) => {
                        let toast = adw::Toast::new(&format!("Failed: {}", e));
                        toast_overlay.add_toast(toast);
                    }
                }
            });
        }
    });

    content.append(&apply_btn);
    page.set_child(Some(&content));
    page
}
