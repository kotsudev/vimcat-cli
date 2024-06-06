pub enum Keys {
    RemoveConfigs,
    InstallConfigs,
    InstallHomebrew,
    InstallGit,
    InstallFzf,
    InstallNerdfonts,
    InstallIterm,
    SetupIterm,
    InstallTmux,
    SetupTmux,
    InstallOhmyzsh,
    SetupOhmyzsh,
    InstallZshsyntax,
    SetupZshsyntax,
    InstallZshsuggestions,
    SetupZshsuggestions,
    InstallPowerlevel10k,
    SetupPowerlevel10k,
    InstallNeovim,
    SetupNeovim,
}

impl Keys {
    pub fn as_str(&self) -> &str {
        match self {
            Keys::RemoveConfigs => "remove configs",
            Keys::InstallConfigs => "install configs",
            Keys::InstallHomebrew => "install homebrew",
            Keys::InstallGit => "install git",
            Keys::InstallFzf => "install fuzzy finder",
            Keys::InstallNerdfonts => "install nerd fonts",
            Keys::InstallIterm => "install iterm",
            Keys::SetupIterm => "setup iterm",
            Keys::InstallTmux => "install tmux",
            Keys::SetupTmux => "setup tmux",
            Keys::InstallOhmyzsh => "install ohmyzsh",
            Keys::SetupOhmyzsh => "setup ohmyzsh",
            Keys::InstallZshsyntax => "install zshsyntax",
            Keys::SetupZshsyntax => "setup zshsyntax",
            Keys::InstallZshsuggestions => "install zshautosuggestions",
            Keys::SetupZshsuggestions => "setup zshautosuggestions",
            Keys::InstallPowerlevel10k => "install powerlevel10k",
            Keys::SetupPowerlevel10k => "setup powerlevel10k",
            Keys::InstallNeovim => "install neovim",
            Keys::SetupNeovim => "setup neovim",
        }
    }
}
