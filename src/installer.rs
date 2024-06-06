use crate::utils::*;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

pub fn download_configs() -> Result<String, std::io::Error> {
    const REPO_URL: &str = "https://github.com/kotsudev/workspace-configs.git";
    let src_directory = format!("{}/workspace-configs", std::env::var("HOME").unwrap());

    println!("step #. download configs");

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

    Ok("download configs".to_string())
}

pub fn cleanup_configs() -> Result<String, std::io::Error> {
    let src_directory = format!("{}/workspace-configs", std::env::var("HOME").unwrap());
    let dest_path = Path::new(&src_directory);

    println!("step #. cleanup configs");

    if !dest_path.exists() {
        return Ok("there is no available configs for cleanup, skipping...".to_string());
    }

    std::fs::remove_dir_all(dest_path)?;
    Ok("cleanup configs".to_string())
}

pub fn install_homebrew() -> Result<String, std::io::Error> {
    let check_output = Command::new("brew").arg("--version").output()?;
    println!("step #. install homebrew");

    if check_output.status.success() {
        return Ok("homebrew is already installed".to_string());
    }

    println!("homebrew is not installed. installing...");
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
    println!("step #. install git");

    let check_output = Command::new("git").arg("-v").output()?;

    if check_output.status.success() {
        return Ok("git is already installed".to_string());
    }

    println!("git is not installed. installing...");
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
    println!("step #. install nerdfonts");

    let fc_list_output = Command::new("fc-list").stdout(Stdio::piped()).spawn()?;

    let check_output = Command::new("grep")
        .arg("JetBrainsMono Nerd Font")
        .stdin(fc_list_output.stdout.unwrap())
        .output()?;

    if check_output.status.success() {
        return Ok("nerdfonts is already installed".to_string());
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
    println!("step #. install iterm");
    let check_output = Command::new("ls").arg("/Applications/iTerm.app").output()?;

    if check_output.status.success() {
        return Ok("iterm, is already installed".to_string());
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
    println!("step #. setup iterm");

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
    println!("step #. install tmux");

    let check_output = Command::new("tmux").arg("-V").output()?;

    if check_output.status.success() {
        return Ok("tmux is already installed".to_string());
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

pub fn install_tpm() -> Result<String, std::io::Error> {
    println!("step #. install tmux plugin manager");

    let check_output = Command::new("ls")
        .arg(format!(
            "{}/.tmux/plugins/tpm",
            std::env::var("HOME").unwrap()
        ))
        .output()?;

    if check_output.status.success() {
        return Ok("tmux plugin manager is already installed".to_string());
    }

    println!("tpm is not installed. installing...");
    let install_output = Command::new("git")
        .args([
            "clone",
            "https://github.com/tmux-plugins/tpm",
            &format!("{}/.tmux/plugins/tpm", std::env::var("HOME").unwrap()),
        ])
        .output()?;

    if !install_output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to install tpm",
        ));
    }

    Ok("tpm installed successfully".to_string())
}

pub fn setup_tmux() -> Result<String, std::io::Error> {
    println!("step #. setup tmux");

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

pub fn install_ohmyzsh() -> Result<String, std::io::Error> {
    println!("step #. install ohmyzsh");

    let check_output = Command::new("ls")
        .arg(format!("{}/.oh-my-zsh", std::env::var("HOME").unwrap()))
        .output()?;

    if check_output.status.success() {
        return Ok("ohmyzsh is already installed".to_string());
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
    println!("step #. setup ohmyzsh");
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

// TODO: Check if it's already installed.
pub fn install_zshsyntax() -> Result<String, std::io::Error> {
    println!("step #. install zsh-syntax-highlighting");

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
    println!("step #. setup zsh-syntax-highlighting");
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

// TODO: Check if it's already installed.
pub fn install_zshautosuggestions() -> Result<String, std::io::Error> {
    println!("step #. install zsh-auto-suggestions");

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
    println!("step #. install powerlevel10k");

    let check_output = Command::new("ls")
        .arg(format!(
            "{}/.oh-my-zsh/custom/themes/powerlevel10k",
            std::env::var("HOME").unwrap()
        ))
        .output()?;

    if check_output.status.success() {
        return Ok("powerlevel10k is already installed".to_string());
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
    println!("step #. setup powerlevel10k");
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
    println!("step #. install fuzzyfinder");

    let check_output = Command::new("fzf").arg("--version").output()?;

    if check_output.status.success() {
        return Ok("fuzzyfinder is already installed".to_string());
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
    println!("step #. install neovim");

    let check_output = Command::new("nvim").arg("-v").output()?;

    if check_output.status.success() {
        return Ok("neovim is already installed".to_string());
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
    println!("step #. setup neovim");

    let check_output = Command::new("git")
        .args([
            "clone",
            "https://github.com/kotsudev/vimcat",
            &format!("{}/.config/nvim", std::env::var("HOME").unwrap()),
        ])
        .output()?;

    if !check_output.status.success() {
        // TODO: Fix this logic as it can crash not only
        // because of existing neovim config.
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "there is already available config located at ~/.config/nvim, please backup it and try again",
        ));
    }

    Ok("neovim setup successfully".to_string())
}
