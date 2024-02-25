mod cli;
use cli::sysflow::Sys;
use cli::gitcli::Commands;
fn main() {
    Sys::system_flow();
    Commands::git_cli();
}