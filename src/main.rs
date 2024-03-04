mod cli;
use cli::{gitcli::Commands, sysflow::Sys};
fn main() {
    Sys::system_flow();
    Sys::githubcli();
    Commands::git_cli();
}
