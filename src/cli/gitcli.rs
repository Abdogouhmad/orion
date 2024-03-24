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
    /// push the changes with commit
    Push {
        /// Commit message
        #[arg(long, short)]
        commit: Option<String>,
    },
}

/// sub command for git cli commands
impl Commands {
    /// Executes a `git_commit` take the option string and return the string for commit
    ///
    /// # Arguments
    ///
    /// * `commit` - An optional string containing the commit message. If `None`, the function will print
    /// an error message and exit with a non-zero status code.
    ///
    /// # Errors
    ///
    /// If the `git commit` command fails, the function will print an error message and exit with a non-zero
    /// status code.
    ///
    /// # Panics
    ///
    /// The function will panic if it fails to execute the `git commit` command.
    fn git_commit(commit: Option<String>) -> String {
        if let Some(commit_message) = commit {
            commit_message
        } else {
            println!("No commit message provided");
            std::process::exit(1);
        }
    }

    /// git_cli managed the oprations
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
                // push sub command
                Commands::Push { commit } => {
                    if let Some(commit_message) = commit {
                        // Add changes
                        let add_result = Execute::run("git", &["add", "."]);
                        if let Err(err) = add_result {
                            eprintln!("Error adding changes: {:?}", err);
                            std::process::exit(1);
                        }

                        // Commit changes
                        let cmt = Commands::git_commit(Some(commit_message.clone()));
                        let commit_result = Execute::run("git", &["commit", "-m", &cmt]);
                        if let Err(err) = commit_result {
                            eprintln!("Error committing changes: {:?}", err);
                            std::process::exit(1);
                        }

                        // Get current branch name
                        let branch_result =
                            Execute::run("git", &["rev-parse", "--abbrev-ref", "HEAD"]);
                        let branch_name = match branch_result {
                            Ok(bytes) => String::from_utf8_lossy(&bytes).trim().to_string(),
                            Err(err) => {
                                eprintln!("Error getting branch name: {:?}", err);
                                std::process::exit(1);
                            }
                        };

                        // Push changes
                        let push_result = Execute::run(
                            "git",
                            &["push", "--set-upstream", "origin", &branch_name],
                        );
                        if push_result.is_err() {
                            eprintln!("Error pushing changes");
                            std::process::exit(1);
                        }
                        println!("{}", Col::print_col(&Col::Magenta, "Code is pushed"));
                    }
                }
            }
        }
    }
}
