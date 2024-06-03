use std::io;
use std::path::Path;
use std::process::Command;

pub fn download_configs() -> Result<(), std::io::Error> {
    println!("Downloading configs");

    // Define constants
    const REPO_URL: &str = "https://github.com/kotsudev/workspace-configs.git";
    let src_directory = format!("{}/workspace-configs", std::env::var("HOME").unwrap());

    // Ensure destination directory exists
    let dest_path = Path::new(&src_directory);
    if !dest_path.exists() {
        println!("Destination path is not found, creating...");
        std::fs::create_dir(&src_directory)?;
    }

    // Run the git clone command
    let status = Command::new("git")
        .args(["clone", REPO_URL, &src_directory])
        .status()?;

    if status.success() {
        println!("Repository cloned successfully to {:?}", dest_path);
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to clone repository",
        ))
    }
}

pub fn cleanup_configs() -> Result<(), std::io::Error> {
    let src_directory = format!("{}/workspace-configs", std::env::var("HOME").unwrap());

    // Remove the destination directory
    let dest_path = Path::new(&src_directory);
    if dest_path.exists() {
        std::fs::remove_dir_all(dest_path)?;
        println!("Repository at {:?} removed successfully", dest_path);
    } else {
        println!("Destination directory does not exist");
    }
    Ok(())
}

pub fn copy(src: &str, dest: &str, file: &str) -> io::Result<()> {
    // Ensure source directory exists
    let src_path = Path::new(src);
    if !src_path.exists() || !src_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Source directory not found",
        ));
    }

    // Ensure destination directory exists
    let dest_path = Path::new(dest);
    if !dest_path.exists() || !dest_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Destination directory not found",
        ));
    }

    // Build full source and destination paths
    let src_file = src_path.join(file);
    let dest_file = dest_path.join(file);

    // Copy file
    std::fs::copy(&src_file, &dest_file)?;

    Ok(())
}
