use std::process::Command;

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

        let install_output = Command::new("brew install git")
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

fn install_iterm() {
    println!("Step 3. Iterm");
    println!("Checking Iterm I is installed...");

    let git_check_output = Command::new("git")
        .arg("-v")
        .output()
        .expect("Failed to check if Git is installed.");

    if !git_check_output.status.success() {
        println!("Git is not installed. Installing...");

        let install_output = Command::new("brew install git")
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

fn install_tmux() {
    println!("Step 4. Tmux");
    println!("Checking if Tmux is installed...");
}

fn install_nerdfonts() {
    println!("Step 5. NerdFonts");
    println!("Checking if NerdFonts is installed...");
}

fn install_ohmyzsh() {
    println!("Step 6. OhMyZsh");
    println!("Checking if OhMyZsh is installed...");
}

fn install_zshsyntax() {
    println!("Step 7. ZshSyntaxHighlighting");
    println!("Checking if ZshSyntaxHighlighting is installed...");
}

fn install_powerlevel10k() {
    println!("Step 8. Powerlevel10k");
    println!("Checking if Powerlevel10k is installed...");
}

fn install_fzf() {
    println!("Step 9. FuzzyFinder");
    println!("Checking if FuzzyFinder is installed...");
}

fn install_neovim() {
    println!("Step 10. Neovim");
    println!("Checking if Neovim is installed...");
}

fn main() {
    println!("Welcome to vimcat cli tool");

    install_homebrew();
    install_fzf()
}
