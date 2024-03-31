use crate::Sys;
use clap::Parser;
use commandcrafter::{color::Col, execute::Execute};
use inquire::{Confirm, InquireError, Select, Text};

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
    /// `git_cli` Execute series of commands
    /// # SubCommands
    /// **clone:** Clone the repo interactivily
    /// **push:** Push the changes to github interactivily
    /// # Examples
    /// ```
    /// whispercli clone
    /// whispercli push
    /// ```
    pub fn git_cli() {
        let args = Sys::parse();
        match args.command {
            Some(command) => Commands::apply_command(&command),
            None => todo!(),
        }
    }

    fn apply_clone() {
        let username = Text::new("Enter the owner of repo: ").prompt();
        let repo = Text::new("Enter the name of the repo: ").prompt();
        let depth = Text::new("Enter the depth of cloning: ")
            .with_default("0")
            .prompt_skippable();

        if let (Ok(username), Ok(repo), Ok(depth)) = (username, repo, depth) {
            let clonefmt = format!("git@github.com:{}/{}.git", username, repo);
            let mut clone_pattern = vec!["clone"];

            // Only add depth option if depth is provided and greater than 0
            if let Some(depth) = depth.as_deref() {
                let depth_int: usize = depth.parse().unwrap_or(0);
                if depth_int > 0 {
                    clone_pattern.push("--depth");
                    clone_pattern.push(depth);
                }
            }

            clone_pattern.push(&clonefmt);

            let res = Execute::run("git", &clone_pattern);
            if res.is_ok() {
                println!("{}", Col::Green.print_col("Clone the repo well"))
            } else if res.is_err() {
                println!("{}", Col::Red.print_col("Clone the repo didn't go well"))
            }
        }
    }

    fn apply_push() {
        let variety_commits = vec![
            "New Improvement to the code base 🚀",
            "Working on new feature 👷‍♂️",
            "Bug is Fix 🐛",
            "Docs are updated 📚",
            "Code is styled 🫠",
            "Codebase is refactored 🏭",
            "Test is updated 🤖",
            "Other changes🙂",
            "Customized Commit 😎",
        ];
        // select option
        let selected_commit = Select::new("Select a commit type", variety_commits).prompt();

        match selected_commit {
            Ok(commit_type) => match commit_type {
                "Customized Commit 😎" => {
                    let customize_commit = Text::new("Please Enter Commit Messege 😎:").prompt();
                    Commands::push_changes(&customize_commit)
                }
                _ => Commands::push_changes(&Ok(commit_type.to_string())),
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    fn apply_release() {
        let tag = Text::new("Enter the version of your app 🚀: ").prompt();
        let msg = Text::new("Message for the tag 😉: ").prompt();
        let confirm_push = Confirm::new("Do you want to push the tag ❓🤔: ")
            .with_default(false)
            .prompt();
        if let (Ok(t), Ok(m), Ok(pt)) = (tag, msg, confirm_push) {
            println!("{}; {}", t, m);
            if pt {
                println!("ok")
            } else if !pt {
                println!("ok bye")
            }
            // let msg_fmt = format!("\"{}\"", m);
            // let res = Execute::run("git", &["tag", "-a", &t, "-m", &msg_fmt]);
            // if res.is_ok() {
            //     println!("{}", Col::Green.print_col("Tag created successfully"))
            // } else if res.is_err() {
            //     println!("{}", Col::Red.print_col("Tag creation failed"))
            // }
        }
    }

    fn apply_command(command: &Commands) {
        match *command {
            Commands::Clone => Commands::apply_clone(),
            Commands::Push => Commands::apply_push(),
            Commands::Release => Commands::apply_release(),
        }
    }

    fn push_changes(commit: &Result<String, InquireError>) {
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
                let branch_result = Execute::run("git", &["rev-parse", "--abbrev-ref", "HEAD"]);
                let branch_name = match branch_result {
                    Ok(bytes) => String::from_utf8_lossy(&bytes).trim().to_string(),
                    Err(err) => {
                        eprintln!("Error getting branch name: {:?}", err);
                        std::process::exit(1);
                    }
                };

                // push the branch head name :')
                let push_result =
                    Execute::run("git", &["push", "--set-upstream", "origin", &branch_name]);
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
