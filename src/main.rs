mod cli;
use clap::Parser;
use cli::coloredcli::get_styles as cli_style;
use cli::{filemanager::FileCreate, gitcli::Commands, sysflow::Syscmd};

/// Whisper CLI tool meant to minimize the amount of written command line in the terminal.
#[derive(Parser, Debug)]
#[command(version = "2.2.2", about, long_about, styles=cli_style())]
pub struct Sys {
    /// List packages that needs to be updated
    #[arg(short, long)]
    list: bool,

    /// update the packages using your fav package manager
    #[arg(short, long)]
    update: bool,
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

    /// sub command for git status
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// create a project based on language type
    #[arg(short, long)]
    pub file: bool,
}

fn main() {
    let args = Sys::parse();

    Syscmd::system_flow(&args);
    Commands::git_cli();
    FileCreate::create_project(&args);
}
