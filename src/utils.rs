use color_eyre::{eyre::bail, Result};
use std::path::Path;
use std::process::Command;
use std::process::ExitStatus;
use std::process::Stdio;

pub fn copy(src: &str, dest: &str, file: &str) -> Result<()> {
    let src_path = Path::new(src);
    if !src_path.exists() || !src_path.is_dir() {
        bail!("copy error: source directory not found");
    }

    let dest_path = Path::new(dest);
    if !dest_path.exists() || !dest_path.is_dir() {
        bail!("copy error: destination directory not found");
    }

    let src_file = src_path.join(file);
    let dest_file = dest_path.join(file);

    std::fs::copy(src_file, dest_file)?;

    Ok(())
}

pub fn executec(command: &str, args: &[&str]) -> Result<ExitStatus> {
    let status = Command::new(command)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    if !status.success() {
        bail!(format!(
            "failed to execute command: {}, with args: {:?}",
            command, args
        ));
    };

    Ok(status)
}
