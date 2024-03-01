use crate::cli;
use clap::Parser;
use cli::sysflow::Sys;
use commandcrafter::{color::Col, execute::Execute};

#[derive(Parser, Debug)]
pub enum Commands {
    /// git status
    #[clap(
        long_about = "git status is a command that shows the status of the files in the working tree"
    )]
    Status,
    /// push changes you made to github
    Push {
        /// the commit message
        #[arg(long, short)]
        commit: Option<String>,
    },

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
                // get the changes you made
                Commands::Status => {
                    let s = Execute::run("git", &["status", "--short"]);
                    Execute::print_into_console(s)
                }
                Commands::Push { commit } => {
                    // TODO: git add
                    // TODO: deal with git commit
                    // TODO: push the changes based on branch
                    println!("your commit is: {:?}", commit)
                }
                Commands::Clone {
                    username,
                    repo,
                    depth,
                } => {
                    if let (Some(u), Some(r), Some(d)) = (username, repo, depth) {
                        if d == "full" {
                            let color = Col::print_col(
                                &Col::Yellow,
                                "hey there you choose the full clone now!",
                            );
                            println!("{}", color);
                            let clonefmt = format!("git@github.com:{}/{}.git", u, r);
                            let r = Execute::run("git", &["clone", &clonefmt]);
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
                        }
                    } else {
                        println!("Username and repo must be provided for the clone command");
                    }
                }
            }
        }
    }
}
