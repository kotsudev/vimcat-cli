use std::io;
use std::path::Path;

pub struct CopyResult {
    pub success: bool,
}

impl CopyResult {
    pub fn success(&self) -> bool {
        self.success
    }
}

pub fn copy(src: &str, dest: &str, file: &str) -> io::Result<CopyResult> {
    let src_path = Path::new(src);
    if !src_path.exists() || !src_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Source directory not found",
        ));
    }

    let dest_path = Path::new(dest);
    if !dest_path.exists() || !dest_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Destination directory not found",
        ));
    }

    let src_file = src_path.join(file);
    let dest_file = dest_path.join(file);

    std::fs::copy(src_file, dest_file)?;

    Ok(CopyResult { success: true })
}

pub fn run_step(step_fn: fn() -> Result<String, std::io::Error>) {
    match step_fn() {
        Ok(r) => println!("Success: {}", r),
        Err(e) => eprintln!("Error: {}", e),
    }
}
