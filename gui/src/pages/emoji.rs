use adw::prelude::*;
use crate::log_window;
use gtk;
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

    let btn_ref = apply_btn.clone();
    let toast_clone = toast_overlay.clone();
    let current_label_clone = current_label.clone();

    apply_btn.connect_clicked(move |_| {
        let parent = match btn_ref.root().and_downcast::<gtk::Window>() {
            Some(w) => w,
            None => return,
        };

        let selected = selected_ref.borrow().clone();
        let Some(package) = selected else { return };

        let old = emoji::get_installed_emoji();
        let mut commands = Vec::new();

        if let Some(ref old_pkg) = old {
            if old_pkg == &package {
                return;
            }
            commands.push((
                "pkexec".to_string(),
                vec![
                    "pacman".to_string(),
                    "-Rds".to_string(),
                    old_pkg.clone(),
                    "--noconfirm".to_string(),
                ],
            ));
        }

        commands.push((
            "pkexec".to_string(),
            vec![
                "pacman".to_string(),
                "-Sy".to_string(),
                package.clone(),
                "--noconfirm".to_string(),
            ],
        ));

        let toast = toast_clone.clone();
        let label = current_label_clone.clone();

        log_window::run_commands(
            &format!("Installing {} emoji set...", package),
            &parent,
            commands,
            move || {
                toast.add_toast(adw::Toast::new(&format!("Installed {} emoji set", package)));
                label.set_text(&format!("Current: {}", package));
            },
        );
    });

    content.append(&apply_btn);
    page.set_child(Some(&content));
    page
}
