use crate::Sys;
use clap::Parser;
use orion_lib::github::{clone, push, release};

#[derive(Parser, Debug)]
pub enum Commands {
    /// clone any repo use help Clone to know more 🫠
    #[clap(long_about = "clone any repo with username + repo's name and choose Full or 1")]
    Clone,
    /// push the changes to the github 😃
    Push,
    /// create release tag through the shell command
    Release,
}

/// sub command for git cli commands
impl Commands {
    pub fn git_cli() {
        let args = Sys::parse();
        if let Some(command) = args.command {
            Commands::apply_command(&command);
        }
        // match args.command {
        //     Some(command) => Commands::apply_command(&command),
        //     None => eprintln!("Out of range"),
        // }
    }

    fn apply_command(command: &Commands) {
        match *command {
            Commands::Clone => clone::apply_clone(),
            Commands::Push => push::apply_push(),
            Commands::Release => release::apply_release(),
        }
    }
}
