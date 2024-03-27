use crate::Sys;
use clap::Parser;
use commandcrafter::{color::Col, execute::Execute};
use inquire::Text;

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

    /// push the changes to the github 😃
    Push,
}

/// sub command for git cli commands
impl Commands {
    /// `git_cli` Execute series of commands
    /// # SubCommands
    /// **clone:** Clone the repo with argument options
    /// - u : username
    /// - r : repo name
    /// - d : depth of the clone it is optional
    /// **push:** Push the changes to github
    /// - c : commit message
    /// # Examples
    /// ```
    /// whispercli clone -u=username -r=reponame
    /// whispercli push
    /// ```

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

                Commands::Push => {
                    let commit = Text::new("Please Enter Commit name 😎:").prompt();
                    match commit {
                        Ok(commit) => {
                            // track the changes :)
                            let add_result = Execute::run("git", &["add", "."]);
                            if let Err(err) = add_result {
                                eprintln!("Error adding changes: {:?}", err);
                                std::process::exit(1);
                            }

                            // commit the changes
                            let commit_result = Execute::run("git", &["commit", "-m", &commit]);
                            if let Err(err) = commit_result {
                                eprintln!("Error committing changes: {:?}", err);
                                std::process::exit(1);
                            }

                            // get the branch the name :/
                            let branch_result =
                                Execute::run("git", &["rev-parse", "--abbrev-ref", "HEAD"]);
                            let branch_name = match branch_result {
                                Ok(bytes) => String::from_utf8_lossy(&bytes).trim().to_string(),
                                Err(err) => {
                                    eprintln!("Error getting branch name: {:?}", err);
                                    std::process::exit(1);
                                }
                            };

                            // push the branch head name :')
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
                        Err(e) => println!("{e}"),
                    }
                }
            }
        }
    }
}
