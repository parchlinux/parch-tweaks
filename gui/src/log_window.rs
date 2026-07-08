use adw::prelude::*;
use gtk::{self, glib};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::time::Duration;

pub struct LogWindow {
    pub window: gtk::Window,
    pub buffer: gtk::TextBuffer,
    pub text_view: gtk::TextView,
}

pub fn open(title: &str, parent: &impl IsA<gtk::Window>) -> LogWindow {
    let header = adw::HeaderBar::builder()
        .title_widget(&gtk::Label::new(Some(title)))
        .build();

    let window = gtk::Window::builder()
        .modal(true)
        .transient_for(parent)
        .default_width(700)
        .default_height(450)
        .build();
    window.set_titlebar(Some(&header));

    let buffer = gtk::TextBuffer::new(None);
    let text_view = gtk::TextView::builder()
        .buffer(&buffer)
        .editable(false)
        .monospace(true)
        .vexpand(true)
        .css_classes(["terminal"])
        .build();

    let scrolled = gtk::ScrolledWindow::builder()
        .child(&text_view)
        .vexpand(true)
        .build();

    window.set_child(Some(&scrolled));
    window.present();

    LogWindow { window, buffer, text_view }
}

pub fn run_commands<F>(
    title: &str,
    parent: &impl IsA<gtk::Window>,
    commands: Vec<(String, Vec<String>)>,
    on_success: F,
) where
    F: Fn() + 'static,
{
    let log = open(title, parent);
    let (tx, rx) = mpsc::channel::<String>();

    let window = log.window.clone();
    let buffer = log.buffer.clone();
    let text_view = log.text_view.clone();

    glib::spawn_future_local(async move {
        loop {
            while let Ok(msg) = rx.try_recv() {
                if msg == "__SUCCESS__" {
                    on_success();
                    continue;
                }
                if msg == "__CLOSE__" {
                    window.close();
                    return;
                }
                let mut end = buffer.end_iter();
                buffer.insert(&mut end, &msg);
                let mark = buffer.create_mark(None, &buffer.end_iter(), false);
                text_view.scroll_to_mark(&mark, 0.0, false, 0.0, 0.0);
            }
            glib::timeout_future(Duration::from_millis(50)).await;
        }
    });

    let tx_clone = tx.clone();
    std::thread::spawn(move || {
        for (cmd, args) in &commands {
            let _ = tx.send(format!("$ {} {}\n", cmd, args.join(" ")));

            let mut child = match Command::new(cmd)
                .args(args.iter().map(|s| s.as_str()))
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
            {
                Ok(c) => c,
                Err(e) => {
                    let _ = tx.send(format!("Error: Failed to spawn: {}\n\n", e));
                    return;
                }
            };

            let stderr_handle = child.stderr.take();
            let stdout_handle = child.stdout.take();

            let tx_err = tx.clone();
            let err_thread = std::thread::spawn(move || {
                if let Some(stderr) = stderr_handle {
                    let reader = BufReader::new(stderr);
                    for line in reader.lines() {
                        if let Ok(line) = line {
                            let _ = tx_err.send(line + "\n");
                        }
                    }
                }
            });

            if let Some(stdout) = stdout_handle {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        let _ = tx.send(line + "\n");
                    }
                }
            }

            let _ = err_thread.join();

            match child.wait() {
                Ok(status) if status.success() => {
                    let _ = tx.send("OK\n\n".to_string());
                }
                Ok(status) => {
                    let code = status.code().map_or("?".to_string(), |c| c.to_string());
                    let _ = tx.send(format!("Failed (code: {})\n\n", code));
                    return;
                }
                Err(e) => {
                    let _ = tx.send(format!("Error: {}\n\n", e));
                    return;
                }
            }

            let _ = tx.send("\n".to_string());
        }

        let _ = tx_clone.send("\nOperation complete. You may close this window.\n".to_string());
        let _ = tx_clone.send("__SUCCESS__".to_string());
    });
}
