use std::io;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

fn download_configs() -> Result<(), std::io::Error> {
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

fn cleanup_configs() -> Result<(), std::io::Error> {
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

fn install_homebrew() {
    println!("Step 1. Homebrew");
    println!("Checking if Homebrew is installed...");

    let brew_check_output = Command::new("brew")
        .arg("--version")
        .output()
        .expect("Failed to check if Homebrew is installed.");

    if !brew_check_output.status.success() {
        println!("Homebrew is not installed. Installing...");

        let install_output = Command::new("/bin/bash")
            .arg("-c")
            .arg(r#"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"#)
            .output()
            .expect("Failed to install Homebrew.");

        if install_output.status.success() {
            println!("Homebrew installed successfully.");
        } else {
            println!("Failed to install Homebrew.");
        }
    } else {
        println!("Homebrew is already installed.");
    }
}

fn install_git() {
    println!("Step 2. Git");
    println!("Checking if Git is installed...");

    let git_check_output = Command::new("git")
        .arg("-v")
        .output()
        .expect("Failed to check if Git is installed.");

    if !git_check_output.status.success() {
        println!("Git is not installed. Installing...");

        let install_output = Command::new("brew")
            .arg("install")
            .arg("git")
            .output()
            .expect("Failed to install Git.");

        if install_output.status.success() {
            println!("Git installed successfully.");
        } else {
            println!("Failed to install Git.");
        }
    } else {
        println!("Git is already installed.");
    }
}

fn install_nerdfonts() {
    println!("Step 5. NerdFonts");
    println!("Checking if NerdFonts is installed...");

    let fc_list_output = Command::new("fc-list")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute fc-list command");

    let nerdfont_check_output = Command::new("grep")
        .arg("JetBrainsMono Nerd Font")
        .stdin(fc_list_output.stdout.unwrap())
        .output()
        .expect("Failed to check if NerdFonts is installed");

    if !nerdfont_check_output.status.success() {
        println!("NerdFonts is not installed. Installing...");

        let install_fonts_output = Command::new("brew")
            .arg("tap")
            .arg("homebrew/cask-fonts")
            .output()
            .expect("Failed to setup homebrew fonts.");
        let install_output = Command::new("brew")
            .arg("install")
            .arg("font-jetbrains-mono-nerd-font")
            .output()
            .expect("Failed to install NerdFonts.");

        if install_output.status.success() {
            println!("NerdFonts installed successfully.");
        } else {
            println!("Failed to install NerdFonts.");
        }
    } else {
        println!("NerdFonts is already installed.");
    }
}

fn install_iterm() {
    println!("Step 3. Iterm");
    println!("Checking Iterm is installed...");

    let iterm_check_output = Command::new("ls")
        .arg("/Applications/iTerm.app")
        .output()
        .expect("Failed to check if Iterm is installed.");

    if !iterm_check_output.status.success() {
        println!("Iterm is not installed. Installing...");

        let install_output = Command::new("brew")
            .arg("install")
            .arg("--cask")
            .arg("iterm2")
            .output()
            .expect("Failed to install Iterm.");

        if install_output.status.success() {
            println!("Iterm installed successfully.");
        } else {
            println!("Failed to install Iterm.");
        }
    } else {
        println!("Iterm is already installed.");
    }
}

fn copy(src: &str, dest: &str, file: &str) -> io::Result<()> {
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

fn setup_iterm() {
    println!("Step 4. Setup Iterm");

    let src_directory = format!(
        "{}/workspace-configs/iterm2",
        std::env::var("HOME").unwrap()
    );
    let dest_directory = format!(
        "{}/Library/Application Support/iTerm2/DynamicProfiles",
        std::env::var("HOME").unwrap()
    );

    match copy(&src_directory, &dest_directory, "main_profile.json") {
        Ok(_) => println!("File copied successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn install_tmux() {
    println!("Step 4. Tmux");
    println!("Checking if Tmux is installed...");

    let tmux_check_output = Command::new("tmux")
        .arg("-V")
        .output()
        .expect("Failed to check if Tmux is installed.");

    if !tmux_check_output.status.success() {
        println!("Tmux is not installed. Installing...");

        let install_output = Command::new("brew")
            .arg("install")
            .arg("tmux")
            .output()
            .expect("Failed to install Tmux.");

        if install_output.status.success() {
            println!("Tmux installed successfully.");
        } else {
            println!("Failed to install Tmux.");
        }
    } else {
        println!("Tmux is already installed.");
    }
}

fn setup_tmux() {
    println!("Step 4. Setup Tmux");

    // Run the git clone command
    let tpm_check_output = Command::new("git")
        .args([
            "clone",
            "https://github.com/tmux-plugins/tpm",
            &format!("{}/.tmux/plugins/tpm", std::env::var("HOME").unwrap()),
        ])
        .output()
        .expect("Failed");

    if tpm_check_output.status.success() {
        println!("Repository cloned successfully");
    }

    let src_directory = format!("{}/workspace-configs/tmux", std::env::var("HOME").unwrap());
    let dest_directory = format!("{}/", std::env::var("HOME").unwrap());

    match copy(&src_directory, &dest_directory, ".tmux.conf") {
        Ok(_) => println!("Tmux setup finished successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("Tmux setup has been finished, to finish the installation open tmux inside iterm and press prefix + I, this will install all configured plugins")
}

// TODO install it at the beginning of the script.
// TODO move setup logic into separate function.
fn install_ohmyzsh() {
    println!("Step 6. OhMyZsh");
    println!("Checking if OhMyZsh is installed...");

    // Install ohmyzsh using curl
    let zsh_check_output = Command::new("sh")
        .args(["-c", "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"])
        .output().expect("Failed installing ohmyzsh");

    if zsh_check_output.status.success() {
        println!("OhMyZsh installed successfully");
    }

    let src_directory = format!("{}/workspace-configs/zsh", std::env::var("HOME").unwrap());
    let dest_directory = format!("{}/", std::env::var("HOME").unwrap());

    match copy(&src_directory, &dest_directory, ".zshrc") {
        Ok(_) => println!("Zsh setup finished successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }
}

// TODO move setup logic into separate function.
fn install_zshsyntax() {
    println!("Step 7. ZshSyntaxHighlighting");
    println!("Checking if ZshSyntaxHighlighting is installed...");

    let install_output = Command::new("brew")
        .arg("install")
        .arg("zsh-syntax-highlighting")
        .output()
        .expect("Failed to install Zsh syntax highlighting.");

    if install_output.status.success() {
        println!("Zsh syntax highlighting installed successfully.");
    } else {
        println!("Failed to install Zsh syntax highlighting.");
    }

    let src_directory = format!(
        "{}/workspace-configs/zsh-syntax-highlighting",
        std::env::var("HOME").unwrap()
    );
    let dest_directory = format!("{}/.zsh/", std::env::var("HOME").unwrap());

    // Ensure destination directory exists
    let dest_path = Path::new(&dest_directory);
    if !dest_path.exists() {
        println!("Destination path is not found, creating...");
        std::fs::create_dir(&dest_directory);
    }

    match copy(
        &src_directory,
        &dest_directory,
        "catppuccin_mocha-zsh-syntax-highlighting.zsh",
    ) {
        Ok(_) => println!("Zsh theme setup finished successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn install_zshautosuggestions() {
    println!("Step 7. ZshAutoSuggestions");
    println!("Checking if ZshAutoSuggestions is installed...");

    let install_output = Command::new("brew")
        .arg("install")
        .arg("zsh-autosuggestions")
        .output()
        .expect("Failed to install Zsh auto suggestions");

    if install_output.status.success() {
        println!("Zsh auto suggestions installed successfully.");
    } else {
        println!("Failed to install Zsh auto suggestions");
    }
}

// TODO save already installed config in backup folder, if it's available.
// TODO move setup logic into separate function.
fn install_powerlevel10k() {
    println!("Step 8. Powerlevel10k");
    println!("Checking if Powerlevel10k is installed...");

    let powerlevel10k_check_output = Command::new("git")
        .args([
            "clone",
            "--depth=1",
            "https://github.com/romkatv/powerlevel10k.git",
            &format!(
                "{}/.oh-my-zsh/custom/themes/powerlevel10k",
                std::env::var("HOME").unwrap()
            ),
        ])
        .output()
        .expect("Failed installing powerlevel10k");

    if powerlevel10k_check_output.status.success() {
        println!("Powerlevel10k installed successfully");
    }

    let src_directory = format!(
        "{}/workspace-configs/powerlevel10k",
        std::env::var("HOME").unwrap()
    );
    let dest_directory = format!("{}/", std::env::var("HOME").unwrap());

    match copy(&src_directory, &dest_directory, ".p10k.zsh") {
        Ok(_) => println!("Powerlevel10k setup finished successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn install_fzf() {
    println!("Step 9. FuzzyFinder");
    println!("Checking if FuzzyFinder is installed...");

    let fzf_check_output = Command::new("fzf")
        .output()
        .expect("Failed to check if Fzf is installed.");

    if !fzf_check_output.status.success() {
        println!("Fzf is not installed. Installing...");

        let install_output = Command::new("brew")
            .arg("install")
            .arg("fzf")
            .output()
            .expect("Failed to install Fzf.");

        if install_output.status.success() {
            println!("Fzf installed successfully.");
        } else {
            println!("Failed to install Fzf.");
        }
    } else {
        println!("Fzf is already installed.");
    }
}

// TODO add backup functionality if there is already neovim distro.
// TODO move setup logic into separate function.
fn install_neovim() {
    println!("Step 10. Neovim");
    println!("Checking if Neovim is installed...");

    let neovim_check_output = Command::new("nvim")
        .arg("-v")
        .output()
        .expect("Failed to check if Neovim is installed.");

    if !neovim_check_output.status.success() {
        let neovim_check_output = Command::new("brew")
            .args(["install", "neovim"])
            .output()
            .expect("Failed installing neovim");

        if neovim_check_output.status.success() {
            println!("Vimcat installed successfully");
        }
    } else {
        println!("Neovim is already installed");
    }

    let vimcat_check_output = Command::new("git")
        .args([
            "clone",
            "https://github.com/kotsudev/vimcat",
            &format!("{}/.config/nvim", std::env::var("HOME").unwrap()),
        ])
        .output()
        .expect("Failed installing vimcat");

    if vimcat_check_output.status.success() {
        println!("Vimcat installed successfully");
    }
}

fn main() {
    println!("Welcome to vimcat cli tool");

    // match cleanup_configs() {
    //     Ok(_) => println!("Cleanup successful"),
    //     Err(e) => eprintln!("Error cleaning up repository: {}", e)
    // };

    // match download_configs() {
    //     Ok(_) => println!("Clone successful"),
    //     Err(e) => eprintln!("Error cloning repository: {}", e),
    // };

    // install_homebrew();
    // install_git();
    // install_nerdfonts();
    // install_iterm();
    // setup_iterm();
    // install_ohmyzsh();
    // install_zshsyntax();
    // install_powerlevel10k();
    // install_tmux();
    // setup_tmux();
    // install_neovim();
    install_zshautosuggestions();
}
