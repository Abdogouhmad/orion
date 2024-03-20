use crate::Sys;
use commandcrafter::{color::Col, execute::Execute, filestore::Filestore};
use std::{env, fs, process};
pub struct Syscmd;

impl Syscmd {
    /// # system_flow
    /// this method intends to operate over many operations such:
    /// * `list`: list the packages that need to be updated within pacman and yay package manager
    /// * `update`: update the packages within pacman and yay both at once
    /// * `weight`: list the weight of each folder within the same directory
    /// * `delete`: delete the log folder which has the logs of the update operation
    pub fn system_flow(args: &Sys) {
        // list option command
        if args.list {
            // list the packages needs to be updated for both
            let p = Execute::run("pacman", &["-Qu", "--color=always"]);
            let y = Execute::run("yay", &["-Qu", "--color=always"]);
            let c = vec![p, y];
            Execute::print_into_console_multiple(c);
        }

        // update command
        if args.update {
            // update packages in both yay and pacman
            let p = Execute::run("sudo", &["pacman", "-Syu", "--noconfirm"]);
            // yay update 2nd
            let y = Execute::run("yay", &["-Syu", "--noconfirm"]);
            let cmb = Filestore::write_combined_to_desktop_log(&[p, y]);
            match cmb {
                Ok(_) => {
                    println!("{}", Col::print_col(&Col::Green, "SEE DESKTOP/LOG"));
                    let _ = Execute::run("notify-send", &["System is updated"]);
                }
                Err(e) => println!(
                    "{} {}",
                    Col::print_col(&Col::Red, "Something went wrong: "),
                    e
                ),
            }
            // clean the cache of package managers
            let _ = Execute::run("paccache", &["-ru"]);
            let _ = Execute::run("sudo", &["pacman", "-Sc"]);
            let _ = Execute::run("yay", &["-Sc"]);
        }

        // weight option command
        if args.weight {
            let w = Execute::run("du", &["-h", "--max-depth=1", ".", "--time"]);
            Execute::print_into_console(w);
        }

        // delete option command
        if args.delete {
            println!(
                "{}",
                Col::print_col(&Col::Yellow, "deleting log folder in process....")
            );
            // create a patten that match with location of the folder
            let d = env::var("HOME").unwrap() + "/Desktop/logs";
            // remove the folder
            let r = fs::remove_dir_all(d);
            // checking if the folder is deleted if not print an error
            if r.is_ok() {
                println!(
                    "{}",
                    Col::print_col(&Col::Green, "log folder deleted successfully")
                );
            } else {
                println!(
                    "{}",
                    Col::print_col(
                        &Col::Red,
                        "log folder deletion failed check if the folder exists"
                    )
                );
                process::exit(1);
            }
        }
    }

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
    /// This function is an entry point for the GitHub CLI tool.
    ///
    /// It parses command line arguments and checks for the presence of a 'status' flag.
    /// If the 'status' flag is present, it prints "hey status" to the console.
    ///
    /// # Examples
    ///
    /// To use this function, you would typically call it from the main function of your CLI tool:
    ///
    /// ```
    ///     github_cli()
    /// ```
    ///
    /// If the 'status' flag is present in the command line arguments, this will print "hey status" to the console.
    pub fn handle_github_cli(args: &Sys) {
        if args.status {
            let s = Execute::run("git", &["status", "--short"]);
            Execute::print_into_console(s)
        }

        if let Some(commit_message) = &args.commit {
            // Add changes
            let add_result = Execute::run("git", &["add", "."]);
            if let Err(err) = add_result {
                eprintln!("Error adding changes: {:?}", err);
                std::process::exit(1);
            }

            // Commit changes
            let cmt = Syscmd::git_commit(Some(commit_message.clone()));
            let commit_result = Execute::run("git", &["commit", "-m", &cmt]);
            if let Err(err) = commit_result {
                eprintln!("Error committing changes: {:?}", err);
                std::process::exit(1);
            }

            // Get current branch name
            let branch_result = Execute::run("git", &["rev-parse", "--abbrev-ref", "HEAD"]);
            let branch_name = match branch_result {
                Ok(bytes) => String::from_utf8_lossy(&bytes).trim().to_string(),
                Err(err) => {
                    eprintln!("Error getting branch name: {:?}", err);
                    std::process::exit(1);
                }
            };

            // Push changes
            let push_result =
                Execute::run("git", &["push", "--set-upstream", "origin", &branch_name]);
            if push_result.is_err() {
                eprintln!("Error pushing changes");
                std::process::exit(1);
            }

            println!("{}", Col::print_col(&Col::Magenta, "Code is pushed"));
        }
    }
}
