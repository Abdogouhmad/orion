use crate::cli;
use clap::Parser;
use cli::sysflow::Sys;
use commandcrafter::execute::Execute;

#[derive(Parser, Debug)]
pub enum Commands {
    /// git status
    #[clap(
        long_about = "git status is a command that shows the status of the files in the working tree"
    )]
    Status,
    /// git add is adding the files to the staging
    Add,

    /// clone any repo use help Clone to know more
    #[clap(long_about = "clone any repo with username + repo's name and choose Full or 1")]
    Clone {
        /// the owner of repo you want to clone
        #[arg(long, short)]
        username: Option<String>,
        /// the name of repo to be cloned
        #[arg(long, short)]
        repo: Option<String>,
        /// depth of the cloning
        #[arg(long, short, default_value = "1")]
        depth: Option<String>,
    },
}

impl Commands {
    pub fn git_cli() {
        let args = Sys::parse();
        if let Some(command) = args.command {
            match command {
                Commands::Status => println!("git status is done"),
                Commands::Add => println!("git add is done"),
                Commands::Clone {
                    username,
                    repo,
                    depth,
                } => {
                    if let (Some(u), Some(r), Some(d)) = (username, repo, depth) {
                        // println!(
                        //    "Cloning repository with username '{}' and repo '{}' and depth '{:#?}",
                        //    u, r, d
                        // );
                        if d == "full" {
                            println!("hey there you choose the full clone now!");
                            let clonefmt = format!("git@github.com:{}/{}.git", u, r);
                            Execute::run("git", &["clone", &clonefmt]);
                            // TODO: you have to create andother module create to return result to check this command
                        }
                    } else {
                        println!("Username and repo must be provided for the clone command");
                    }
                }
            }
        }
    }
}
