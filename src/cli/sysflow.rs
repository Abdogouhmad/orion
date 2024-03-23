use crate::Sys;
use commandcrafter::{color::Col, execute::Execute, filestore::Filestore};
use inquire::MultiSelect;
use std::{
    env, fs,
    process::{self, exit},
};

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
            // vector str of options
            let packagemanger = vec!["pacman", "yay"];
            // map the vec options
            let packager = MultiSelect::new(
                "choose the package manager you want to update 😄 🆙",
                packagemanger,
            )
            .prompt();
            // match the options
            match packager {
                Ok(pckg) => {
                    for p in pckg {
                        match p {
                            // TODO: fun for updating
                            "pacman" => self::Syscmd::arch_update("pacman"),
                            "yay" => self::Syscmd::arch_update("yay"),
                            _ => eprintln!("out of range"),
                        }
                    }
                }
                Err(_) => eprintln!("error"),
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

    /// update_pacman
    fn arch_update(name: &str) {
        let pac_flag = ["pacman", "-Syu", "--noconfirm"];
        let yay_flag = ["-Syu", "--noconfirm"];
        if name.contains("pacman") {
            let pac = Execute::run("sudo", &pac_flag);
            let pac_log = Filestore::write_into_desktop(&pac, "/pacman.log");
            match pac_log {
                Ok(_) => {
                    let _ = Execute::run("notify-send", &["Pacman packages are updated"]);
                }
                Err(e) => {
                    eprintln!("{} : {}", Col::Red.print_col("Something went wrong"), e);
                    let _ = Execute::run("notify-send", &["Error updateding"]);
                    exit(1);
                }
            }
        } else if name.contains("yay") {
            let yay = Execute::run("yay", &yay_flag);
            let yay_log = Filestore::write_into_desktop(&yay, "/yay.log");
            match yay_log {
                Ok(_) => {
                    let _ = Execute::run("notify-send", &["Yay packages are updated"]);
                }
                Err(e) => {
                    eprintln!("{} : {}", Col::Red.print_col("Something went wrong"), e);
                    let _ = Execute::run("notify-send", &["Error updateding"]);
                    exit(1);
                }
            }
        }
        let _ = [
            Execute::run("paccache", &["-ru"]),
            Execute::run("sudo", &["pacman", "-Sc"]),
            Execute::run("yay", &["-Sc"]),
        ];
    }
    // TODO: Add more package managers in futures
}
