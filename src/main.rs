mod errors;
mod installer;
mod utils;

use installer::*;

use color_eyre::eyre::WrapErr;
use color_eyre::Result;
use std::thread::sleep;
use std::time::Duration;

type Step = fn() -> Result<()>;

fn main() -> Result<()> {
    errors::install_hooks()?;

    let steps: &[(&str, Step)] = &[
        ("cleanning configs", cleanup_configs),
        ("download configs", download_configs),
        ("installing fzf", install_fzf),
        ("installing nerdfonts", install_nerdfonts),
        ("installing iterm", install_iterm),
        ("setting up item", setup_iterm),
        ("installing tmux", install_tmux),
        ("setting up tmux", setup_tmux),
        ("installing ohmyzsh", install_ohmyzsh),
        ("setting up ohmyzsh", setup_ohmyzsh),
        ("installing zshsyntax", install_zshsyntax),
        ("setting up zshsyntax", setup_zshsyntax),
        ("installing zsh authsuggestions", install_zshautosuggestions),
        ("installing powerlevel10k", install_powerlevel10k),
        ("setting up powerlevel10k", setup_powerlevel10k),
        ("installing neovim", install_neovim),
        ("setting up neovim", setup_neovim),
        ("cleanning configs", cleanup_configs),
    ];

    for (step_number, (step_name, step_fn)) in steps.iter().enumerate() {
        println!("step {}: {}", step_number, step_name);
        step_fn().wrap_err(format!("step: {}", step_name))?;
        sleep(Duration::from_millis(1000));
    }

    println!("vimcat has been installed successfully!");
    println!(
        "to finish the installation open tmux inside iterm and press `ctrl + x + I`, 
        this will install all configured plugins"
    );
    println!("tip: to apply updated tmux config use `ctrl + x + r`");

    Ok(())
}
