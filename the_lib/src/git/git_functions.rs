use anyhow::{Context, Error, Result};
use git2::{Cred, IndexAddOption, RemoteCallbacks, Repository, Signature};
use std::{env, path::Path};
use tokio::task;

use crate::{colprintln, eclprintln};

pub struct Git;

impl Git {
    pub async fn push_changes(commit: &str) -> Result<(), Error> {
        // Convert the commit string slice to an owned String
        let commit_owned = commit.to_string();

        task::spawn_blocking(move || -> Result<()> {
            // Check if the current directory is the root of the project ..
            let repo = match Repository::open("./") {
                Ok(repo) => repo,
                Err(_) => {
                    eclprintln!("<r> Failed to open repository. Make sure you are in the root of your project.</>");
                    return Err(anyhow::anyhow!("Failed to open repository"));
                }
            };

            // Get the index
            let mut index = repo.index().context("Failed to get the index file")?;

            // Add all changes to the index
            index.add_all(["."].iter(), IndexAddOption::DEFAULT, None)?;
            if let Err(err) = index.write() {
                eprintln!("Error adding changes: {:?}", err);
                std::process::exit(1);
            }

            // Check if there are any changes to commit
            let tree_id = index.write_tree()?;
            let tree = repo.find_tree(tree_id)?;
            let head = repo.head()?;
            let parent_commit = head.peel_to_commit()?;

            if tree.id() == parent_commit.tree_id() {
                colprintln!("<g> Nothing to commit. Working tree clean.</>");
                return Ok(());
            }

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
                &commit_owned,
                &tree,
                &[&parent_commit],
            )
            .context("Can't commit at the moment")?;

            // Get the branch name
            let branch_name = repo
                .head()
                .expect("Failed to get HEAD")
                .shorthand()
                .expect("Failed to get branch name")
                .to_string();

            // Find remote
            let mut remote = repo
                .find_remote("origin")
                .context("Can't find remote origin")?;

            // Create a callback
            let mut callbacks = RemoteCallbacks::new();
            callbacks.credentials(|_url, username_from_url, _allowed_types| {
                Cred::ssh_key(
                    username_from_url.unwrap_or("git"),
                    None,
                    Path::new(&format!("{}/.ssh/id_ed25519", env::var("HOME").unwrap())), // TODO: problem is here
                    None,
                )
            });

            // Create a git option
            let mut opts = git2::PushOptions::new();
            opts.remote_callbacks(callbacks);

            // Push the changes
            let result = remote.push(&[&format!("refs/heads/{}", branch_name)], Some(&mut opts));
            match result {
                Ok(_) => colprintln!("<g> Code is pushed</>"),
                Err(e) => eclprintln!("<r> Failed to push: {e}</>"),
            };

            Ok(())
        })
        .await
        .context("Failed to push changes")?
    }
}
