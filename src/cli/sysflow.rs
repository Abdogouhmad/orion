use clap::Parser;
use commandcrafter::{color::Col, execute::Execute, filestore::Filestore};
use std::{env, fs, process};
// for subcommand that is gitcli.rs
use crate::Commands;

/// Whisper CLI tool meant to minimize the amount of written command line in the terminal.
#[derive(Parser, Debug)]
#[command(version = "1.0.0", about, long_about)]
pub struct Sys {
    /// List pacman or yay packages that need to be updated
    #[arg(short, long)]
    list: Option<String>,

    /// update the package either pacman, yay or both
    #[arg(
        short,
        long,
        long_help = "this command option give the ability to choose the package manager to operate either pacman, yay or both of them"
    )]
    update: Option<String>,
    /// measure the weight of folders
    #[arg(
        short,
        long,
        long_help = "measure the weight of every single folder within the same directory "
    )]
    weight: bool,

    /// delete the log
    #[arg(
        short,
        long,
        long_help = "delete the log folder that contains the update system package within /Desktop/log"
    )]
    delete: bool,

    /// sub command for git status
    #[command(subcommand)]
    pub command: Option<Commands>,
}

impl Sys {
    /// # system_flow
    /// this method intends to operate over many operations such:
    /// * `list`: list the packages that need to be updated within pacman and yay package manager
    /// * `update`: update the packages within pacman and yay or both at once
    /// * `weight`: list the weight of each folder within the same directory
    /// * `delete`: delete the log folder which has the logs of the update operation
    pub fn system_flow() {
        let args = Sys::parse();

        // list option command
        if let Some(list) = args.list {
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
        if let Some(update) = args.update {
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
                process::exit(1)
            }
        }
    }
}
