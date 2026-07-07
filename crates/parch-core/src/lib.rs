pub mod emoji;
pub mod editor;
pub mod aur;
pub mod fonts;
pub mod shell;
pub mod system;

use anyhow::{Context, Result};
use std::process::Command;

pub fn run_command(program: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .with_context(|| format!("failed to execute: {} {}", program, args.join(" ")))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!("command failed: {}", stderr))
    }
}

pub async fn run_pkexec(args: &[&str]) -> Result<String> {
    let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    tokio::task::spawn_blocking(move || {
        let output = Command::new("pkexec")
            .args(&args)
            .output()
            .context("failed to execute pkexec")?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("pkexec failed: {}", stderr))
        }
    })
    .await?
}
