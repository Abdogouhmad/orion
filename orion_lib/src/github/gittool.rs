use commandcrafter::{color::Col, execute::Execute};
use inquire::{Confirm, InquireError, Select, Text};

/// A struct representing a set of Git tools.
pub struct GitTool;

impl GitTool {
    /// Apply a git push operation based on user input.
    ///
    /// This method prompts the user to select a commit type from a predefined list
    /// or allows them to input a custom commit message. It then adds and commits
    /// the changes to the local repository and pushes them to the remote repository.
    pub fn apply_push() {
        let variety_commits = vec![
            "New Improvement to the code base 🚀",
            "Working on new feature 👷‍♂️",
            "Bug is Fix 🐛",
            "Docs are updated 📚",
            "Code is styled 🫠",
            "Codebase is refactored 🏭",
            "Optimize the code",
            "Clean the code",
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
                    GitTool::push_changes(&customize_commit)
                }
                _ => GitTool::push_changes(&Ok(commit_type.to_string())),
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    /// Apply a git release operation based on user input.
    ///
    /// This method prompts the user to input a tag version and message for the release.
    /// It then creates a tag with the provided version and message, and optionally
    /// pushes the tag to the remote repository if the user confirms.
    pub fn apply_release() {
        let tag = Text::new("Enter the version of your app 🚀: ").prompt();
        let msg = Text::new("Message for the tag 😉: ").prompt();
        let confirm_push = Confirm::new("Do you want to push the tag ❓🤔: ")
            .with_default(false)
            .prompt();
        if let (Ok(t), Ok(m), Ok(pt)) = (tag, msg, confirm_push) {
            let msg_fmt = format!("\"{}\"", m);
            let res = Execute::exe("git", &["tag", "-a", &t, "-m", &msg_fmt]);
            if res.is_ok() {
                println!("{}", Col::Green.print_col("Tag created successfully"));
                if pt {
                    let tag_push = ["push", "origin", &t];
                    let _ = Execute::exe("git", &tag_push);
                } else if !pt {
                    println!("ok bye")
                }
            } else if res.is_err() {
                println!("{}", Col::Red.print_col("Tag creation failed"))
            }
        }
    }

    /// Apply a git clone operation based on user input.
    ///
    /// This method prompts the user to input the owner's username and the repository
    /// name they want to clone. Additionally, it allows the user to specify the depth
    /// of the clone operation. It then clones the specified repository to the local
    /// machine using the provided parameters.
    pub fn apply_clone() {
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

            let res = Execute::exe("git", &clone_pattern);
            if res.is_ok() {
                println!("{}", Col::Green.print_col("Clone the repo well"))
            } else if res.is_err() {
                println!("{}", Col::Red.print_col("Clone the repo didn't go well"))
            }
        }
    }

    /// Helper function to push changes to the remote repository.
    ///
    /// This method takes a commit message as input and performs the following steps:
    /// 1. Adds all changes to the staging area.
    /// 2. Commits the changes with the provided commit message.
    /// 3. Retrieves the current branch name.
    /// 4. Pushes the committed changes to the remote repository.
    ///
    /// # Arguments
    ///
    /// * `commit` - A `Result` containing the commit message provided by the user.
    ///
    /// # Panics
    ///
    /// This method panics if any of the git commands fail during execution.
    fn push_changes(commit: &Result<String, InquireError>) {
        match commit {
            Ok(commit) => {
                // track the changes :)
                let add_result = Execute::exe("git", &["add", "."]);
                if let Err(err) = add_result {
                    eprintln!("Error adding changes: {:?}", err);
                    std::process::exit(1);
                }

                // commit the changes
                let commit_result = Execute::exe("git", &["commit", "-m", &commit]);
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
                    Execute::exe("git", &["push", "--set-upstream", "origin", &branch_name]);
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
