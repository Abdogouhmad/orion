mod cli;
use clap::Parser;
use cli::coloredcli::get_styles as cli_style;
use cli::{filemanager::FileCreate, gitcli::Commands, sysflow::Syscmd};

/// Whisper CLI tool meant to minimize the amount of written command line in the terminal.
#[derive(Parser, Debug)]
#[command(version = "1.2.0", about, long_about, styles=cli_style())]
pub struct Sys {
    /// List pacman or yay packages that need to be updated
    #[arg(short, long)]
    list: Option<String>,

    /// update the package either pacman, yay or both
    #[arg(
        short,
        long,
        long_help = "this command option give the ability to choose the package manager to operate either pacman, yay or both of them"
    )]
    update: Option<String>,
    /// measure the weight of folders
    #[arg(
        short,
        long,
        long_help = "measure the weight of every single folder within the same directory "
    )]
    weight: bool,

    /// delete the log
    #[arg(
        short,
        long,
        long_help = "delete the log folder that contains the update system package within /Desktop/log"
    )]
    delete: bool,

    /// provide the files that are in change git status
    #[arg(short, long)]
    status: bool,

    /// push changes you made to github
    #[arg(
        long,
        short,
        long_help = "capturing the commit msg through assign it as string -c=\"your msg\" "
    )]
    commit: Option<String>,

    /// sub command for git status
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Test ping command
    #[arg(short, long)]
    pub ping: bool,
}

fn main() {
    let args = Sys::parse();

    Syscmd::system_flow(&args);
    Syscmd::handle_github_cli(&args);
    Commands::git_cli();
    FileCreate::handle_more_commands(&args);
}
