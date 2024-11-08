use crate::utils::*;
use color_eyre::{eyre::bail, Result};
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

pub fn download_configs() -> Result<()> {
    const REPO_URL: &str = "https://github.com/kotsudev/workspace-configs.git";
    let src_directory = format!("{}/workspace-configs", std::env::var("HOME").unwrap());

    if !Path::new(&src_directory).exists() {
        std::fs::create_dir(&src_directory)?;
    }

    executec("git", &["clone", REPO_URL, &src_directory])?;

    Ok(())
}

pub fn cleanup_configs() -> Result<()> {
    let src_directory = format!("{}/workspace-configs", std::env::var("HOME").unwrap());
    let dest_path = Path::new(&src_directory);

    if !dest_path.exists() {
        return Ok(());
    }

    std::fs::remove_dir_all(dest_path)?;

    Ok(())
}

pub fn install_homebrew() -> Result<()> {
    let check_status = executec("brew", &["--version"])?;

    if check_status.success() {
        return Ok(());
    }

    executec(
        "/bin/bash",
        &[
            "-c",
            r#"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"#,
        ],
    )?;

    Ok(())
}

pub fn install_git() -> Result<()> {
    let check_status = executec("git", &["-v"])?;

    if check_status.success() {
        return Ok(());
    }

    executec("brew", &["install", "git"])?;

    Ok(())
}

pub fn install_nerdfonts() -> Result<()> {
    let fc_list_output = Command::new("fc-list").stdout(Stdio::piped()).spawn()?;
    let check_output = Command::new("grep")
        .arg("JetBrainsMono Nerd Font")
        .stdin(fc_list_output.stdout.unwrap())
        .output()?;

    if check_output.status.success() {
        return Ok(());
    }

    executec("brew", &["tap", "homebrew/cask-fonts"])?;
    executec("brew", &["install", "font-jetbrains-mono-nerd-font"])?;

    Ok(())
}

pub fn install_iterm() -> Result<()> {
    let check_output = executec("ls", &["/Applications/iTerm.app"])?;

    if check_output.success() {
        return Ok(());
    }

    executec("brew", &["install", "--cask", "iterm2"])?;

    Ok(())
}

pub fn setup_iterm() -> Result<()> {
    let src_directory = format!(
        "{}/workspace-configs/iterm2",
        std::env::var("HOME").unwrap()
    );
    let dest_directory = format!(
        "{}/Library/Application Support/iTerm2/DynamicProfiles",
        std::env::var("HOME").unwrap()
    );

    copy(&src_directory, &dest_directory, "main_profile.json")?;

    Ok(())
}

pub fn install_tmux() -> Result<()> {
    let check_output = executec("tmux", &["-V"])?;

    if check_output.success() {
        return Ok(());
    }

    executec("brew", &["install", "tmux"])?;

    Ok(())
}

pub fn setup_tmux() -> Result<()> {
    let src_directory = format!("{}/workspace-configs/tmux", std::env::var("HOME").unwrap());
    let dest_directory = format!("{}/", std::env::var("HOME").unwrap());

    copy(&src_directory, &dest_directory, ".tmux.conf")?;

    Ok(())
}

pub fn install_ohmyzsh() -> Result<()> {
    let ohmyzsh_directory = format!("{}/.oh-my-zsh", std::env::var("HOME").unwrap());
    let check_output = executec("ls", &[&ohmyzsh_directory])?;

    if check_output.success() {
        return Ok(());
    }

    executec("sh", &[
        "-c", 
        "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)",
    ])?;

    Ok(())
}

pub fn setup_ohmyzsh() -> Result<()> {
    let src_directory = format!("{}/workspace-configs/zsh", std::env::var("HOME").unwrap());
    let dest_directory = format!("{}/", std::env::var("HOME").unwrap());

    copy(&src_directory, &dest_directory, ".zshrc")?;

    Ok(())
}

// TODO: Check if it's already installed.
pub fn install_zshsyntax() -> Result<()> {
    executec("brew", &["install", "zsh-syntax-highlighting"])?;

    Ok(())
}

pub fn setup_zshsyntax() -> Result<()> {
    let src_directory = format!(
        "{}/workspace-configs/zsh-syntax-highlighting",
        std::env::var("HOME").unwrap()
    );
    let dest_directory = format!("{}/.zsh/", std::env::var("HOME").unwrap());
    let dest_path = Path::new(&dest_directory);

    if !dest_path.exists() {
        std::fs::create_dir(&dest_directory)?;
    }

    copy(
        &src_directory,
        &dest_directory,
        "catppuccin_mocha-zsh-syntax-highlighting.zsh",
    )?;

    Ok(())
}

// TODO: Check if it's already installed.
pub fn install_zshautosuggestions() -> Result<()> {
    executec("brew", &["install", "zsh-autosuggestions"])?;

    Ok(())
}

pub fn install_powerlevel10k() -> Result<()> {
    let powerlevel10k_directory = format!(
        "{}/.oh-my-zsh/custom/themes/powerlevel10k",
        std::env::var("HOME").unwrap()
    );
    let check_output = executec("ls", &[&powerlevel10k_directory])?;

    if check_output.success() {
        return Ok(());
    }

    executec(
        "git",
        &[
            "clone",
            "--depth=1",
            "https://github.com/romkatv/powerlevel10k.git",
            &powerlevel10k_directory,
        ],
    )?;

    Ok(())
}

pub fn setup_powerlevel10k() -> Result<()> {
    let src_directory = format!(
        "{}/workspace-configs/powerlevel10k",
        std::env::var("HOME").unwrap()
    );
    let dest_directory = format!("{}/", std::env::var("HOME").unwrap());

    copy(&src_directory, &dest_directory, ".p10k.zsh")?;

    Ok(())
}

pub fn install_fzf() -> Result<()> {
    let check_output = executec("fzf", &["--version"])?;

    if check_output.success() {
        return Ok(());
    }

    executec("brew", &["install", "fzf"])?;

    Ok(())
}

pub fn install_neovim() -> Result<()> {
    let check_output = executec("nvim", &["-v"])?;

    if check_output.success() {
        return Ok(());
    }

    executec("brew", &["install", "neovim"])?;

    Ok(())
}

pub fn setup_neovim() -> Result<()> {
    let check_output = executec(
        "git",
        &[
            "clone",
            "https://github.com/kotsudev/vimcat",
            &format!("{}/.config/nvim", std::env::var("HOME").unwrap()),
        ],
    )?;

    if !check_output.success() {
        // TODO: Fix this logic as it can crash not only
        // because of existing neovim config.
        bail!("there is already available config located at ~/.config/nvim, please back it up and try again")
    }

    Ok(())
}
