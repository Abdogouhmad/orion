mod cli;
use cli::{gitcli::Commands, sysflow::Sys};
fn main() {
    Sys::system_flow();
    Sys::handle_github_cli();
    Commands::git_cli();
}
