mod installer;
mod utils;

use installer::*;
use utils::*;

fn main() {
    println!("Welcome to vimcat cli tool");

    let steps: &[fn() -> Result<String, std::io::Error>] = &[
        cleanup_configs,
        download_configs,
        install_homebrew,
        install_git,
        install_fzf,
        install_nerdfonts,
        install_iterm,
        setup_iterm,
        install_tmux,
        setup_tmux,
        install_ohmyzsh,
        setup_ohmyzsh,
        install_zshsyntax,
        setup_zshsyntax,
        install_zshautosuggestions,
        install_powerlevel10k,
        setup_powerlevel10k,
        install_neovim,
        setup_neovim,
    ];

    for step_fn in steps {
        run_step(*step_fn);
    }
}
