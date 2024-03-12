use crate::Sys;
use clap::Parser;
use commandcrafter::{color::Col, execute::Execute};

#[derive(Parser, Debug)]
pub enum Commands {
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

/// sub command for git cli commands
impl Commands {
    pub fn git_cli() {
        let args = Sys::parse();
        if let Some(command) = args.command {
            match command {
                Commands::Clone {
                    username,
                    repo,
                    depth,
                } => {
                    if let (Some(u), Some(r)) = (username, repo) {
                        let clonefmt = format!("git@github.com:{}/{}.git", u, r);
                        // clone based on value provided either 1 or full or none which will clone as full
                        // TODO: make the depth to be customized based on provided number
                        let clone_args: Vec<&str> = match depth {
                            Some(d) if d == "full" => vec!["clone", "--depth", "full", &clonefmt],
                            Some(d) if d == "1" => vec!["clone", "--depth", "1", &clonefmt],
                            // TODO: parse from String to &str
                            _ => vec!["clone", &clonefmt],
                        };
                        let r = Execute::run("git", &clone_args);
                        match r {
                            Ok(_) => {
                                println!("{}", Col::print_col(&Col::Green, "repo is cloned"))
                            }
                            Err(e) => println!(
                                "{} {}",
                                Col::print_col(&Col::Red, "something happened: "),
                                e
                            ),
                        };
                    } else {
                        println!("Username and repo must be provided for the clone command");
                    }
                }
            }
        }
    }
}
