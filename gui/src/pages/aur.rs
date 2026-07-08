use adw::prelude::*;
use crate::log_window;
use gtk;
use parch_core::aur;

pub fn build_page(toast_overlay: &adw::ToastOverlay) -> adw::NavigationPage {
    let page = adw::NavigationPage::builder()
        .title("AUR Helper")
        .tag("aur")
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
        .label("<big>AUR Helper</big>")
        .use_markup(true)
        .halign(gtk::Align::Start)
        .build();
    content.append(&title);

    let desc = gtk::Label::builder()
        .label("Choose your preferred AUR helper for Parch Linux")
        .halign(gtk::Align::Start)
        .build();
    desc.add_css_class("dim-label");
    content.append(&desc);

    let current = aur::get_installed_aur_helper();
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
        .title("Available AUR Helpers")
        .build();

    let selected_ref = std::cell::RefCell::new(current.clone());

    for helper in aur::AUR_HELPERS {
        let row = adw::ActionRow::builder()
            .title(helper.name)
            .subtitle(if helper.is_default { "Default for Parch Linux" } else { "" })
            .activatable(false)
            .build();

        let radio = gtk::CheckButton::builder()
            .active(current.as_deref() == Some(helper.name))
            .valign(gtk::Align::Center)
            .build();

        row.add_suffix(&radio);

        let helper_name = helper.name.to_string();
        let selected_clone = selected_ref.clone();

        radio.connect_toggled(move |is_active| {
            if is_active.is_active() {
                *selected_clone.borrow_mut() = Some(helper_name.clone());
            }
        });

        group.add(&row);
    }

    content.append(&group);

    content.append(&gtk::Separator::new(gtk::Orientation::Horizontal));

    let apply_btn = gtk::Button::builder()
        .label("Switch AUR Helper")
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
        let Some(name) = selected else { return };

        let Some(helper) = aur::AUR_HELPERS.iter().find(|h| h.name == name) else { return };

        let current = aur::get_installed_aur_helper();
        let mut commands = Vec::new();

        if let Some(ref current_name) = current {
            if current_name == &name {
                return;
            }
            commands.push((
                "pkexec".to_string(),
                vec![
                    "pacman".to_string(),
                    "-Rds".to_string(),
                    current_name.clone(),
                    "--noconfirm".to_string(),
                ],
            ));
        }

        commands.push((
            "pkexec".to_string(),
            vec![
                "pacman".to_string(),
                "-Sy".to_string(),
                helper.package.to_string(),
                "--noconfirm".to_string(),
            ],
        ));

        let toast = toast_clone.clone();
        let label = current_label_clone.clone();
        let name_display = name.clone();

        log_window::run_commands(
            &format!("Switching to {}...", name),
            &parent,
            commands,
            move || {
                toast.add_toast(adw::Toast::new(&format!("Switched to {}", name_display)));
                label.set_text(&format!("Current: {}", name_display));
            },
        );
    });

    content.append(&apply_btn);
    page.set_child(Some(&content));
    page
}
