use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{self, glib};

use crate::pages::{aur, editors, emoji, fonts, shell, system_info};

pub fn build_ui(app: &adw::Application) {
    let window = AppWindow::new(app);
    window.present();
}

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct AppWindow;

    #[glib::object_subclass]
    impl ObjectSubclass for AppWindow {
        const NAME: &'static str = "ParchTweaksWindow";
        type Type = super::AppWindow;
        type ParentType = adw::ApplicationWindow;
    }

    impl ObjectImpl for AppWindow {}
    impl WidgetImpl for AppWindow {}
    impl WindowImpl for AppWindow {}
    impl ApplicationWindowImpl for AppWindow {}
    impl AdwApplicationWindowImpl for AppWindow {}
}

glib::wrapper! {
    pub struct AppWindow(ObjectSubclass<imp::AppWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root;
}

impl AppWindow {
    pub fn new(app: &adw::Application) -> Self {
        let win: Self = glib::Object::builder()
            .property("application", app)
            .property("title", "Parch Linux Tweaks")
            .property("default-width", 900)
            .property("default-height", 600)
            .build();

        let menu_btn = gtk::MenuButton::builder()
            .icon_name("open-menu-symbolic")
            .tooltip_text("Menu")
            .build();

        let menu = gtk::gio::Menu::new();
        let about_item = gtk::gio::MenuItem::new(Some("About"), Some("app.about"));
        menu.append_item(&about_item);

        let popover = gtk::PopoverMenu::from_model(Some(&menu));
        menu_btn.set_popover(Some(&popover));

        let header = adw::HeaderBar::builder()
            .title_widget(&gtk::Label::new(Some("Parch Linux Tweaks")))
            .build();

        header.pack_end(&menu_btn);

        let toast_overlay = adw::ToastOverlay::new();

        let split_view = adw::NavigationSplitView::builder()
            .min_sidebar_width(180.0)
            .max_sidebar_width(260.0)
            .build();

        let sidebar = gtk::ListBox::builder()
            .css_classes(["navigation-sidebar"])
            .build();

        let stack = gtk::Stack::new();
        stack.set_transition_type(gtk::StackTransitionType::Crossfade);
        stack.set_transition_duration(200);

        let toast_ref = toast_overlay.clone();

        let page_data = vec![
            ("face-smile-symbolic", "Emoji", "emoji", emoji::build_page(&toast_ref)),
            ("system-software-install-symbolic", "AUR Helper", "aur", aur::build_page(&toast_ref)),
            ("accessories-text-editor-symbolic", "Editors", "editors", editors::build_page(&toast_ref)),
            ("font-color-symbolic", "Fonts", "fonts", fonts::build_page(&toast_ref)),
            ("utilities-terminal-symbolic", "Shell", "shell", shell::build_page(&toast_ref)),
            ("dialog-information-symbolic", "System Info", "system_info", system_info::build_page()),
        ];

        for (i, (icon_name, label, tag, page)) in page_data.iter().enumerate() {
            stack.add_named(page, Some(tag));

            let icon = gtk::Image::from_icon_name(*icon_name);
            icon.set_pixel_size(20);

            let label_widget = gtk::Label::builder()
                .label(*label)
                .xalign(0.0)
                .build();

            let box_ = gtk::Box::builder()
                .spacing(12)
                .margin_top(6)
                .margin_bottom(6)
                .margin_start(12)
                .margin_end(12)
                .build();
            box_.append(&icon);
            box_.append(&label_widget);

            let row = gtk::ListBoxRow::builder()
                .child(&box_)
                .activatable(true)
                .build();

            sidebar.append(&row);

            if i == 0 {
                sidebar.select_row(Some(&row));
            }
        }

        let stack_ref = stack.clone();
        let split_ref = split_view.clone();
        let page_data_clone = page_data.clone();

        sidebar.connect_row_activated(move |listbox, row| {
            listbox.select_row(Some(row));
            let idx = row.index() as usize;
            if let Some((_, _, tag, _)) = page_data_clone.get(idx) {
                stack_ref.set_visible_child_name(tag);
                if split_ref.is_collapsed() {
                    split_ref.set_show_content(true);
                }
            }
        });

        let sidebar_page = adw::NavigationPage::builder()
            .child(&sidebar)
            .build();

        let content_page = adw::NavigationPage::builder()
            .child(&stack)
            .build();

        split_view.set_sidebar(Some(&sidebar_page));
        split_view.set_content(Some(&content_page));

        toast_overlay.set_child(Some(&split_view));

        let outer_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        outer_box.append(&header);
        outer_box.append(&toast_overlay);

        win.set_content(Some(&outer_box));
        win
    }
}
