mod cli;
use clap::Parser;
use cli::{filemanager::MoreCommands, gitcli::Commands, sysflow::Sys};
fn main() {
    let args = Sys::parse();
    Sys::system_flow();
    Sys::handle_github_cli();
    Commands::git_cli();
    MoreCommands::handle_more_commands(&args);
}
