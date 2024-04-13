use crate::Sys;
use clap::Parser;
use orion_lib::{github::gittool::GitTool, zipper::zip::ZipF};

#[derive(Parser, Debug)]
pub enum Commands {
    /// clone any repo use help Clone to know more 🫠
    #[clap(long_about = "clone any repo with username + repo's name and choose Full or 1")]
    Clone,
    /// push the changes to the github 😃
    Push,
    /// create release tag through the shell command
    Release,
    /// zip your folder
    Zip {
        /// the source that willing to be zipped
        #[arg(long, short)]
        source: String,
        /// the name of zipped asset
        #[clap(long, short)]
        output: String,
    },
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
        match command {
            Commands::Clone => GitTool::apply_clone(),
            Commands::Push => GitTool::apply_push(),
            Commands::Release => GitTool::apply_release(),
            Commands::Zip { source, output } => {
                // Call apply_zip with references to source and output
                let _ = ZipF::apply_zip(source, output);
            }
        }
    }

    // fn apply_command(command: &Commands) {
    //     match *command {
    //         Commands::Clone => GitTool::apply_clone(),
    //         Commands::Push => GitTool::apply_push(),
    //         Commands::Release => GitTool::apply_release(),
    //         Commands::Zip { source, output } => ZipF::apply_zip(source, output),
    //     }
    // }
}
