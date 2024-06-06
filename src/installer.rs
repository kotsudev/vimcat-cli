use crate::keys::*;
use crate::utils::*;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

pub fn download_configs() -> Result<String, std::io::Error> {
    const REPO_URL: &str = "https://github.com/kotsudev/workspace-configs.git";
    let src_directory = format!("{}/workspace-configs", std::env::var("HOME").unwrap());

    println!("{}", Keys::InstallConfigs.as_str());

    if !Path::new(&src_directory).exists() {
        println!("destination path is not found, creating...");
        std::fs::create_dir(&src_directory)?;
    }

    let status = Command::new("git")
        .args(["clone", REPO_URL, &src_directory])
        .status()?;

    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to download configs",
        ));
    };

    Ok(Keys::InstallConfigs.as_str().to_string())
}

pub fn cleanup_configs() -> Result<String, std::io::Error> {
    let src_directory = format!("{}/workspace-configs", std::env::var("HOME").unwrap());
    let dest_path = Path::new(&src_directory);

    println!("{}", Keys::RemoveConfigs.as_str());

    if !dest_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "destination directory does not exist",
        ));
    }

    std::fs::remove_dir_all(dest_path)?;
    Ok(Keys::RemoveConfigs.as_str().to_string())
}

pub fn install_homebrew() -> Result<String, std::io::Error> {
    let check_output = Command::new("brew").arg("--version").output()?;
    println!("Step 1. Homebrew");
    println!("Checking if Homebrew is installed...");

    if check_output.status.success() {
        return Ok(format!(
            "{} skipped, homebrew is already installed",
            Keys::InstallHomebrew.as_str()
        ));
    }

    println!("Homebrew is not installed. Installing...");
    let install_output = Command::new("/bin/bash")
        .arg("-c")
        .arg(r#"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"#)
        .output()?;

    if !install_output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to install homebrew",
        ));
    }

    Ok("homebrew installed successfully.".to_string())
}

pub fn install_git() -> Result<String, std::io::Error> {
    println!("Step 2. Git");
    println!("Checking if Git is installed...");

    let check_output = Command::new("git").arg("-v").output()?;

    if check_output.status.success() {
        return Ok(format!(
            "{}, is already installed",
            Keys::InstallGit.as_str()
        ));
    }

    println!("Git is not installed. Installing...");
    let install_output = Command::new("brew").arg("install").arg("git").output()?;

    if !install_output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to install git",
        ));
    };

    Ok("git installed successfully".to_string())
}

pub fn install_nerdfonts() -> Result<String, std::io::Error> {
    println!("Step 5. NerdFonts");
    println!("Checking if NerdFonts is installed...");

    let fc_list_output = Command::new("fc-list").stdout(Stdio::piped()).spawn()?;

    let check_output = Command::new("grep")
        .arg("JetBrainsMono Nerd Font")
        .stdin(fc_list_output.stdout.unwrap())
        .output()?;

    if check_output.status.success() {
        return Ok(format!(
            "{}, is already installed",
            Keys::InstallNerdfonts.as_str()
        ));
    }

    println!("NerdFonts is not installed. Installing...");
    let fonts_output = Command::new("brew")
        .arg("tap")
        .arg("homebrew/cask-fonts")
        .output()?;

    if !fonts_output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to tap cask-fonts",
        ));
    }

    let install_output = Command::new("brew")
        .arg("install")
        .arg("font-jetbrains-mono-nerd-font")
        .output()?;

    if !install_output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to install nerdfonts",
        ));
    }

    Ok("nerdfonts installed successfully".to_string())
}

pub fn install_iterm() -> Result<String, std::io::Error> {
    println!("Step 3. Iterm");
    println!("Checking Iterm is installed...");

    let check_output = Command::new("ls").arg("/Applications/iTerm.app").output()?;

    if check_output.status.success() {
        return Ok(format!(
            "{}, is already installed",
            Keys::InstallIterm.as_str()
        ));
    }

    println!("Iterm is not installed. Installing...");
    let install_output = Command::new("brew")
        .arg("install")
        .arg("--cask")
        .arg("iterm2")
        .output()?;

    if !install_output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to install iterm",
        ));
    }

    Ok("iterm installed successfully".to_string())
}

pub fn setup_iterm() -> Result<String, std::io::Error> {
    println!("Step 4. Setup Iterm");

    let src_directory = format!(
        "{}/workspace-configs/iterm2",
        std::env::var("HOME").unwrap()
    );
    let dest_directory = format!(
        "{}/Library/Application Support/iTerm2/DynamicProfiles",
        std::env::var("HOME").unwrap()
    );

    let copy_output = copy(&src_directory, &dest_directory, "main_profile.json")?;

    if !copy_output.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to setup iterm",
        ));
    }

    Ok("iterm setup successfully".to_string())
}

pub fn install_tmux() -> Result<String, std::io::Error> {
    println!("Step 4. Tmux");
    println!("Checking if Tmux is installed...");

    let check_output = Command::new("tmux").arg("-V").output()?;

    if check_output.status.success() {
        return Ok(format!(
            "{}, is already installed",
            Keys::InstallTmux.as_str()
        ));
    }

    println!("Tmux is not installed. Installing...");
    let install_output = Command::new("brew").arg("install").arg("tmux").output()?;

    if !install_output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to install tmux",
        ));
    }

    Ok("tmux installed successfully".to_string())
}

pub fn setup_tmux() -> Result<String, std::io::Error> {
    println!("Step 4. Setup Tmux");

    let check_output = Command::new("git")
        .args([
            "clone",
            "https://github.com/tmux-plugins/tpm",
            &format!("{}/.tmux/plugins/tpm", std::env::var("HOME").unwrap()),
        ])
        .output()?;

    if !check_output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to install tmux tpm",
        ));
    }

    let src_directory = format!("{}/workspace-configs/tmux", std::env::var("HOME").unwrap());
    let dest_directory = format!("{}/", std::env::var("HOME").unwrap());

    let copy_output = copy(&src_directory, &dest_directory, ".tmux.conf")?;

    if !copy_output.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to setup tmux",
        ));
    }

    Ok("to finish the installation open tmux inside iterm and press prefix + I, this will install all configured plugins".to_string())
}

// TODO install it at the beginning of the script.
pub fn install_ohmyzsh() -> Result<String, std::io::Error> {
    println!("Step 6. OhMyZsh");
    println!("Checking if OhMyZsh is installed...");

    let check_output = Command::new("ls")
        .arg(format!("{}/.oh-my-zsh", std::env::var("HOME").unwrap()))
        .output()?;

    if check_output.status.success() {
        return Ok(format!(
            "{}, is already installed",
            Keys::InstallOhmyzsh.as_str()
        ));
    }

    println!("ohmyzsh is not installed. Installing...");
    let install_output = Command::new("sh")
        .args(["-c", "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"])
        .output()?;

    if !install_output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to install ohmyzsh",
        ));
    }

    Ok("ohmyzsh installed successfully".to_string())
}

pub fn setup_ohmyzsh() -> Result<String, std::io::Error> {
    let src_directory = format!("{}/workspace-configs/zsh", std::env::var("HOME").unwrap());
    let dest_directory = format!("{}/", std::env::var("HOME").unwrap());

    let copy_output = copy(&src_directory, &dest_directory, ".zshrc")?;

    if !copy_output.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to setup ohmyzsh",
        ));
    }

    Ok("ohmyzsh setup successfully".to_string())
}

// TODO move setup logic into separate function.
pub fn install_zshsyntax() -> Result<String, std::io::Error> {
    println!("Step 7. ZshSyntaxHighlighting");
    println!("Checking if ZshSyntaxHighlighting is installed...");

    let install_output = Command::new("brew")
        .arg("install")
        .arg("zsh-syntax-highlighting")
        .output()?;

    if !install_output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to install zsh-syntax-highlighting",
        ));
    }

    Ok("zsh-syntax-highlighting installed successfully".to_string())
}

pub fn setup_zshsyntax() -> Result<String, std::io::Error> {
    let src_directory = format!(
        "{}/workspace-configs/zsh-syntax-highlighting",
        std::env::var("HOME").unwrap()
    );
    let dest_directory = format!("{}/.zsh/", std::env::var("HOME").unwrap());

    let dest_path = Path::new(&dest_directory);
    if !dest_path.exists() {
        println!("Destination path is not found, creating...");
        std::fs::create_dir(&dest_directory)?;
    }

    let copy_output = copy(
        &src_directory,
        &dest_directory,
        "catppuccin_mocha-zsh-syntax-highlighting.zsh",
    )?;

    if !copy_output.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to setup zsh-syntax-highlighting",
        ));
    }

    Ok("zsh-syntax-highlighting setup successfully".to_string())
}

pub fn install_zshautosuggestions() -> Result<String, std::io::Error> {
    println!("Step 7. ZshAutoSuggestions");
    println!("Checking if ZshAutoSuggestions is installed...");

    let install_output = Command::new("brew")
        .arg("install")
        .arg("zsh-autosuggestions")
        .output()?;

    if !install_output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to install zsh-autosuggestions",
        ));
    }

    Ok("zsh-autosuggestions installed successfully".to_string())
}

pub fn install_powerlevel10k() -> Result<String, std::io::Error> {
    println!("Step 8. Powerlevel10k");
    println!("Checking if Powerlevel10k is installed...");

    let check_output = Command::new("ls")
        .arg(format!(
            "{}/.oh-my-zsh/custom/themes/powerlevel10k",
            std::env::var("HOME").unwrap()
        ))
        .output()?;

    if check_output.status.success() {
        return Ok(format!(
            "{}, is already installed",
            Keys::InstallPowerlevel10k.as_str()
        ));
    }

    let check_output = Command::new("git")
        .args([
            "clone",
            "--depth=1",
            "https://github.com/romkatv/powerlevel10k.git",
            &format!(
                "{}/.oh-my-zsh/custom/themes/powerlevel10k",
                std::env::var("HOME").unwrap()
            ),
        ])
        .output()?;

    if !check_output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to install powerlevel10k",
        ));
    }

    Ok("powerlevel10k is installed successfully".to_string())
}

pub fn setup_powerlevel10k() -> Result<String, std::io::Error> {
    let src_directory = format!(
        "{}/workspace-configs/powerlevel10k",
        std::env::var("HOME").unwrap()
    );
    let dest_directory = format!("{}/", std::env::var("HOME").unwrap());

    let copy_output = copy(&src_directory, &dest_directory, ".p10k.zsh")?;

    if !copy_output.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to setup powerlevel10k",
        ));
    }

    Ok("powerlevel10k setup successfully".to_string())
}

pub fn install_fzf() -> Result<String, std::io::Error> {
    println!("Step 9. FuzzyFinder");
    println!("Checking if FuzzyFinder is installed...");

    let check_output = Command::new("fzf").output()?;

    if check_output.status.success() {
        return Ok(format!(
            "{}, is already installed",
            Keys::InstallFzf.as_str()
        ));
    }

    println!("Fzf is not installed. Installing...");
    let install_output = Command::new("brew").arg("install").arg("fzf").output()?;

    if !install_output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to install fzf",
        ));
    }

    Ok("fzf installed successfully".to_string())
}

pub fn install_neovim() -> Result<String, std::io::Error> {
    println!("Step 10. Neovim");
    println!("Checking if Neovim is installed...");

    let check_output = Command::new("nvim").arg("-v").output()?;

    if check_output.status.success() {
        return Ok(format!(
            "{}, is already installed",
            Keys::InstallNeovim.as_str()
        ));
    }

    println!("neovim is not installed. Installing...");
    let check_output = Command::new("brew").args(["install", "neovim"]).output()?;

    if !check_output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to install fzf",
        ));
    }

    Ok("neovim installed successfully".to_string())
}

pub fn setup_neovim() -> Result<String, std::io::Error> {
    println!("Step 10. Setup Neovim");

    let check_output = Command::new("git")
        .args([
            "clone",
            "https://github.com/kotsudev/vimcat",
            &format!("{}/.config/nvim", std::env::var("HOME").unwrap()),
        ])
        .output()?;

    if !check_output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to install fzf",
        ));
    }

    Ok("neovim setup successfully".to_string())
}
