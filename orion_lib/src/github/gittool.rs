use anyhow::{Context, Result};
use commandcrafter::{color::Col, execute::Execute};
use git2::{Cred, IndexAddOption, RemoteCallbacks, Repository, Signature};
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
                        .expect("The Commit functionality Failed See how to fix --->");
                }
                _ => GitTool::push_changes(&Ok(commit_type.to_string()))
                    .expect("The Commit functionality Failed See how to fix --->"),
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
                username_from_url.unwrap_or("git"),
                None,
                Path::new(&format!("{}/.ssh/id_ed25519", env::var("HOME").unwrap())),
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
    fn push_changes(commit: &Result<String, InquireError>) -> anyhow::Result<()> {
        match commit {
            Ok(commit) => {
                // Open the repository
                let repo = Repository::open("./")
                    .context("Push in the root of your project in order to get index")?;

                // Get the index
                let mut index = repo.index().context("Failed to get the index file")?;

                // Add all changes to the index
                index.add_all(["."].iter(), IndexAddOption::DEFAULT, None)?;

                // Write the index to disk
                index.write_tree()?;

                // Commit the changes
                let tree_id = index.write_tree()?;
                let tree = repo.find_tree(tree_id)?;
                let head = repo.head()?.peel_to_commit()?;

                // Retrieve signature from config
                let config = repo.config().context("Can't find ~/.gitconfig")?;
                let name = config.get_string("user.name").context("No user set")?;
                let email = config.get_string("user.email").context("No email set")?;
                let signature = Signature::now(&name, &email).context("Null config")?;

                // Commit the changes
                repo.commit(
                    Some("HEAD"),
                    &signature,
                    &signature,
                    commit,
                    &tree,
                    &[&head],
                )
                .context("Can't commit at the moment")?;

                // Get the branch name
                let branch_name = repo.head()?.shorthand().unwrap_or("HEAD").to_string();
                // find remote
                let remote = repo
                    .find_remote("origin")
                    .context("Can't find remote origin");
                // create a call back
                let mut callbacks = RemoteCallbacks::new();
                callbacks.credentials(|_url, username_from_url, _allowed_types| {
                    Cred::ssh_key(
                        username_from_url.unwrap_or("git"),
                        None,
                        Path::new(&format!("{}/.ssh/id_ed25519", env::var("HOME").unwrap())),
                        None,
                    )
                });
                // create a git option
                let mut opts = git2::PushOptions::new();
                opts.remote_callbacks(callbacks);
                remote?
                    .push(&[&format!("refs/heads/{}", branch_name)], Some(&mut opts))
                    .context("Failed to push to remote")?;

                // println!("Code is pushed successfully");
                // // Push changes to remote
                // Execute::exe("git", &["push", "--set-upstream", "origin", &branch_name])?;

                println!("{}", Col::print_col(&Col::Green, "Code is pushed"));
            }
            Err(e) => println!("{}", e),
        }
        Ok(())
    }
}
