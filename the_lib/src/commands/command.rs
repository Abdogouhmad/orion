use clap::Parser;
use std::io::Error;

use crate::{compressor::Zip, git::Git, Orn};
#[derive(Parser, Debug)]
pub enum OrnCommands {
    /// push code to your git repo
    Push {
        /// the commit message
        #[arg(long, short)]
        commit: String,
    },
    /// zip the compressed folders
    Zip {
        /// source to be zipped
        #[arg(long, short)]
        src: String,
        /// output name
        #[arg(long, short)]
        out: String,
    },
}

// TODO: create a function that will match the enum fields
// call it in empty param function
impl OrnCommands {
    pub async fn handle_subcommands(args: &Orn) -> Result<(), Error> {
        if let Some(cmd) = &args.orncommand {
            OrnCommands::match_subcommands(cmd)
                .await
                .unwrap_or_default();
        }
        Ok(())
    }

    async fn match_subcommands(cmd: &OrnCommands) -> Result<(), anyhow::Error> {
        match cmd {
            OrnCommands::Push { commit } => {
                Git::push_changes(commit).await?;
                Ok(())
            }

            OrnCommands::Zip { src, out } => Ok(Zip::apply_zip(src, out).await.unwrap()),
        }
    }
}
