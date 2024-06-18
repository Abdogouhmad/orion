use commandcrafter::{color::Col, execute::Execute};
use git2::{Cred, IndexAddOption, RemoteCallbacks, Repository};
use inquire::{Confirm, InquireError, Select, Text};
use std::{env, path::Path};

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
            "Customized Commit 😎",
            "New Improvement to the code base 🚀",
            "README updated",
            "Working on new feature 👷‍♂️",
            "Bug is Fix 🐛",
            "Docs are updated 📚",
            "Code is styled 🫠",
            "Codebase is refactored 🏭",
            "The code is optimized :smile:",
            "The Code is cleaned",
            "Updating tests 🤖",
            "Other changes🙂",
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
        let repo_path = Text::new("Enter The path where the repo to be cloned: ").prompt();
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key(
                username_from_url.unwrap(),
                None,
                Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
                None,
            )
        });

        // Prepare fetch options.
        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(callbacks);

        // Prepare builder.
        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fo);

        // format the clone
        if let (Ok(username), Ok(repo), Ok(repo_path)) = (username, repo, repo_path) {
            let clone_fmt = format!("git@github.com:{}/{}.git", username, repo);
            // Clone the project.
            let rst = builder.clone(&clone_fmt, Path::new(&repo_path));
            match rst {
                Ok(_) => println!("{}", Col::Green.print_col("Clone successful")),
                Err(e) => {
                    eprintln!("{}", Col::Red.print_col(&format!("Clone failed: {}", e)));
                    std::process::exit(1);
                }
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
                // let add_result = Execute::exe("git", &["add", "."]);
                // open a repo
                let repo = Repository::open("./").expect("failed to open");
                // get the index
                let mut indx = repo.index().expect("Can't get the index file");
                let _ = indx.add_all(["*"].iter(), IndexAddOption::DEFAULT, None);
                let add_result = indx.write();
                if let Err(err) = add_result {
                    eprintln!("Error adding changes: {:?}", err);
                    std::process::exit(1);
                }

                // commit the changes
                let commit_result = Execute::exe("git", &["commit", "-m", commit]);
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
