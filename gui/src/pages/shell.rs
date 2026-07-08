use adw::prelude::*;
use crate::log_window;
use gtk;
use parch_core::shell;

pub fn build_page(toast_overlay: &adw::ToastOverlay) -> adw::NavigationPage {
    let page = adw::NavigationPage::builder()
        .title("Shell")
        .tag("shell")
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
        .label("<big>Shell Configuration</big>")
        .use_markup(true)
        .halign(gtk::Align::Start)
        .build();
    content.append(&title);

    let desc = gtk::Label::builder()
        .label("Install pre-configured shell setups")
        .halign(gtk::Align::Start)
        .build();
    desc.add_css_class("dim-label");
    content.append(&desc);

    let current = shell::get_current_shell();
    let current_label = gtk::Label::builder()
        .label(&format!(
            "Current shell: {}",
            current.as_deref().unwrap_or("unknown")
        ))
        .halign(gtk::Align::Start)
        .build();
    content.append(&current_label);

    content.append(&gtk::Separator::new(gtk::Orientation::Horizontal));

    let group = adw::PreferencesGroup::builder()
        .title("Available Shell Configurations")
        .build();

    let selected_ref = std::cell::RefCell::new(None::<(String, String)>);

    for config in shell::SHELL_CONFIGS {
        let row = adw::ActionRow::builder()
            .title(config.name)
            .subtitle(config.description)
            .activatable(false)
            .build();

        let radio = gtk::CheckButton::builder()
            .valign(gtk::Align::Center)
            .build();

        row.add_suffix(&radio);

        let config_name = config.name.to_string();
        let config_cmd = config.install_cmd.to_string();
        let selected_clone = selected_ref.clone();

        radio.connect_toggled(move |is_active| {
            if is_active.is_active() {
                *selected_clone.borrow_mut() = Some((config_name.clone(), config_cmd.clone()));
            }
        });

        group.add(&row);
    }

    content.append(&group);

    content.append(&gtk::Separator::new(gtk::Orientation::Horizontal));

    let apply_btn = gtk::Button::builder()
        .label("Install Shell Config")
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
        let Some((ref name, ref cmd)) = selected else { return };

        let toast = toast_clone.clone();
        let label = current_label_clone.clone();
        let name_clone = name.clone();
        let cmd_clone = cmd.clone();

        log_window::run_commands(
            &format!("Installing {}...", name),
            &parent,
            vec![
                (
                    "pkexec".to_string(),
                    vec![
                        "pacman".to_string(),
                        "-Sy".to_string(),
                        "zsh".to_string(),
                        "--noconfirm".to_string(),
                    ],
                ),
                (
                    "bash".to_string(),
                    vec!["-c".to_string(), cmd_clone],
                ),
            ],
            move || {
                toast.add_toast(adw::Toast::new(&format!("Installed {}", name_clone)));
                label.set_text(&format!("Current: {}", name_clone));
            },
        );
    });

    content.append(&apply_btn);
    page.set_child(Some(&content));
    page
}
