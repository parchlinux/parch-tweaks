use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub os_name: String,
    pub os_version: String,
    pub kernel: String,
    pub shell: String,
    pub emoji_set: Option<String>,
    pub aur_helper: Option<String>,
}

pub fn get_system_info() -> SystemInfo {
    let os_release = parse_os_release();
    SystemInfo {
        os_name: os_release
            .get("PRETTY_NAME")
            .cloned()
            .unwrap_or_else(|| "Parch Linux".to_string()),
        os_version: os_release
            .get("VERSION_ID")
            .cloned()
            .unwrap_or_else(|| "unknown".to_string()),
        kernel: crate::run_command("uname", &["-r"]).unwrap_or_else(|_| "unknown".to_string()),
        shell: std::env::var("SHELL").unwrap_or_else(|_| "unknown".to_string()),
        emoji_set: crate::emoji::get_installed_emoji(),
        aur_helper: crate::aur::get_installed_aur_helper(),
    }
}

fn parse_os_release() -> HashMap<String, String> {
    let mut map = HashMap::new();
    if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if let Some((key, value)) = line.split_once('=') {
                let value = value.trim_matches('"').trim();
                map.insert(key.to_string(), value.to_string());
            }
        }
    }
    map
}
