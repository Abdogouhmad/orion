use crate::Sys;
use commandcrafter::{color::Col, execute::Execute, filestore::Filestore};
use std::{env, fs, process};
pub struct Syscmd;

impl Syscmd {
    /// # system_flow
    /// this method intends to operate over many operations such:
    /// * `list`: list the packages that need to be updated within pacman and yay package manager
    /// * `update`: update the packages within pacman and yay or both at once
    /// * `weight`: list the weight of each folder within the same directory
    /// * `delete`: delete the log folder which has the logs of the update operation
    pub fn system_flow(args: &Sys) {
        // list option command
        if let Some(list) = &args.list {
            if list == "pacman" {
                // list the packages needs to be updated for pacman
                let p = Execute::run("pacman", &["-Qu", "--color=always"]);
                Execute::print_into_console(p);
            } else if list == "yay" {
                // list the packages needs to be updated for yay
                let y = Execute::run("yay", &["-Qu", "--color=always"]);
                Execute::print_into_console(y);
            } else if list == "both" {
                // list the packages needs to be updated for both
                let p = Execute::run("pacman", &["-Qu", "--color=always"]);
                let y = Execute::run("yay", &["-Qu", "--color=always"]);
                let c = vec![p, y];
                Execute::print_into_console_multiple(c);
            }
        }

        // update option command
        if let Some(update) = &args.update {
            if update == "pacman" {
                // update pacman packages
                let r = Execute::run("sudo", &["pacman", "-Syyu", "--noconfirm"]);
                match r {
                    Ok(_) => println!("{}", Col::print_col(&Col::Green, "Pacman is updated")),
                    Err(e) => println!(
                        "{} {}",
                        Col::print_col(&Col::Red, "Pacman is not updated: "),
                        e
                    ),
                }
            } else if update == "yay" {
                // update yay packages
                let r = Execute::run("yay", &["-Syyu", "--noconfirm"]);
                match r {
                    Ok(_) => println!("{}", Col::print_col(&Col::Green, "Yay is updated")),
                    Err(e) => println!(
                        "{} {}",
                        Col::print_col(&Col::Red, "Yay is not updated: "),
                        e
                    ),
                }
            } else if update == "both" {
                // update packages in both yay and pacman
                let p = Execute::run("sudo", &["pacman", "-Syyu", "--noconfirm"]);
                // yay update 2nd
                let y = Execute::run("yay", &["-Syyu", "--noconfirm"]);
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
                let _ = Execute::run("paccache", &["-ru"]);
                let _ = Execute::run("sudo", &["pacman", "-Scc"]);
                let _ = Execute::run("yay", &["-Scc"]);
            }
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
    /// If the 'status' flag is present in the command line arguments, this will print "hey status" to the console.    ///
    pub fn handle_github_cli(args: &Sys) {
        // TODO: status command
        // TODO: push command which has git add commit and push
        // TODO: fix pushing to github
        if args.status {
            let s = Execute::run("git", &["status", "--short"]);
            Execute::print_into_console(s)
        }
        if let Some(c) = &args.commit {
            let result = Execute::run("git", &["rev-parse", "--abbrev-ref", "HEAD"]);
            let br = match result {
                Ok(bytes) => String::from_utf8(bytes).expect("Invalid UTF-8"),
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    std::process::exit(1);
                }
            };
            let pushing = format!("{}", &br);
            let _ = Execute::run("git", &["add", "."]);
            let cmt = Syscmd::git_commit(Some(c.clone()));
            println!("{}", cmt);
            let _ = Execute::run("git", &["commit", "-m", &cmt]);
            let r = Execute::run("git", &["push", "--set-upstream", "origin", &pushing]);
            if r.is_ok() {
                println!("{}", Col::print_col(&Col::Magenta, "Code is pushed"));
            } else {
                println!(
                    "{}",
                    Col::print_col(&Col::Red, "Error happened during pushing")
                );
                std::process::exit(1)
            }
        }
    }
}
