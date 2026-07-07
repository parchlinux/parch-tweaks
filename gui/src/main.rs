mod app;
mod pages;

use adw::prelude::*;

fn main() {
    let app = adw::Application::builder()
        .application_id("org.parchlinux.parch-tweaks")
        .build();

    app.connect_activate(app::build_ui);

    app.connect_startup(|app| {
        let about_action = gtk::gio::ActionEntry::builder("about")
            .activate(|app: &adw::Application, _, _| {
                let about = adw::AboutWindow::builder()
                    .application(app)
                    .application_name("Parch Linux Tweaks")
                    .application_icon("parch-tweaks")
                    .developer_name("Parch Linux")
                    .version("0.1.0")
                    .developers(vec!["Parch Linux Team"])
                    .copyright("© 2026 Parch Linux")
                    .license_type(gtk::License::Gpl30)
                    .website("https://parchlinux.com")
                    .issue_url("https://github.com/parchlinux/parch-tweaks/issues")
                    .build();
                about.present();
            })
            .build();

        app.add_action_entries([about_action]);
        app.set_accels_for_action("app.about", &["F1"]);
    });

    app.run();
}
