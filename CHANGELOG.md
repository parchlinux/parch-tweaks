# Changelog

## [0.1.1] - 2026-07-08

### Added
- Real-time log window (`gui/src/log_window.rs`) for all install operations, streaming stdout/stderr line-by-line via `Command::spawn` + `BufReader`.
- Dependency checking (`editor::check_deps`) before editor config installs.
- Persistence of installed editor config state via `~/.config/parch-tweaks/installed_config.txt`.

### Changed
- Bumped all crate versions from `0.1.0` to `0.1.1`.
- Editor config installs (`install_neovim_config`, `install_emacs_config`) are now synchronous — removed `async`, removed `run_pkexec`/`pacman` calls, use plain `git clone` to user config directories.
- All page install buttons (emoji, AUR, fonts, shell, editors) now open a `LogWindow` instead of using `glib::spawn_future_local`.
- Emoji and AUR pages now remove the old package before installing the new one (two `pkexec` commands instead of one).
- Shell page installs `zsh` via pkexec before running the shell config command.
- Editor config rows show a check mark icon after successful install, and restore the check mark on next launch.
### Fixed
- **Tokio runtime panic**: `gui/src/main.rs` now creates a `tokio::runtime::Runtime` and calls `.enter()`, so `tokio::task::spawn_blocking` calls in `parch-core` no longer panic.
- **System info GTK thread issue**: `gui/src/pages/system_info.rs` replaced `glib::spawn_future_local` + `tokio::task::spawn_blocking` with plain `std::thread::spawn` for launching system binaries.
- **AdwNavigationPage missing-title warnings**: Added `.title("")` to sidebar and content `NavigationPage` builders in `gui/src/app.rs`.

### Removed
- CLI/TUI binary (`ptt`) — removed `"cli"` from workspace members in root `Cargo.toml`.

### Notes
- The `__CLOSE__` / `__SUCCESS__` signal messages are still present in `log_window.rs` but `__CLOSE__` is never sent (window stays open until user dismisses it).
- The `cli/` directory and its `Cargo.toml` still exist on disk but are no longer built by the workspace.
