mod installer;
mod utils;

use installer::*;

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
