#![allow(unused_imports)]
#![allow(dead_code)]

use clap::{Parser, Subcommand, Args};
use commandcrafter::{color::Col, execute::Execute};

/// Whisper CLI tool meant to minimize the amount of written command line in the terminal.
#[derive(Parser, Debug)]
#[command(version = "1.0.0", about, long_about)]
pub struct Sys {
    /// List pacman or yay packages that need to be update
    #[arg(short, long)]
    list: Option<String>,

    /// update the package either pacman, yay or both
    #[arg(
        short,
        long,
        long_help = "this command option give the ability to choose the package manager to operate either pacman, yay or both of them"
    )]
    update: Option<String>,
    /// measure the wight of folders
    #[arg(
        short,
        long,
        long_help = "measure the wight of every single folder within the same directory "
    )]
    wight: bool,

    /// delete the log
    #[arg(
        short,
        long,
        long_help = "delete the log folder that contains the update system package within /Desktop/log"
    )]
    delete: bool,

    /// sub command for git status
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// git status
    #[clap(long_about = "git status is a command that shows the status of the files in the working tree")]
    Status,
    /// git add is adding the files to the staging
    Add,
}
impl Sys {
    /// # system_flow
    /// this method intends to operate over many operations such:
    /// * `list`: list the packages that need to be updated within pacman and yay package manager
    /// * `update`: update the packages within pacman and yay or both at once
    /// * `wight`: list the wight of each folder within the same directory
    /// * `delete`: delete the log folder which has the logs of the update operation
    pub fn system_flow() {
        let args = Sys::parse();

        // list option command
        if let Some(list) = args.list {   
            if list == "pacman" {
                let p = Execute::run("pacman", &["-Qu", "--color=always"]);
                Execute::print_into_console(&p);
            } else if list == "yay" {
                let y = Execute::run("yay", &["-Qu", "--color=always"]);
                Execute::print_into_console(&y);
            } else if list == "both" {
                let p = Execute::run("pacman", &["-Qu", "--color=always"]);
                let y = Execute::run("yay", &["-Qu", "--color=always"]);
                let c = &[p, y].concat();
                Execute::print_into_console(&c);
            }
        }

        // update option command
        if let Some(update) = args.update {
            if update == "pacman" {
                println!("pacman");
            } else if update == "yay" {
                println!("yay");
            } else if update == "both" {
                println!("both");
            }
        }

        // wight option command
        if args.wight {
            let w = Execute::run("du", &["-h", "--max-depth=1", ".", "--time"]);
            Execute::print_into_console(&w);
        }

        // delete option command
        if args.delete {
            println!("delete log folder in process....")
        }
        // sub command
        if let Some(command) = args.command {
            match command {
                Commands::Status => println!("git status is done"),
                Commands::Add => println!("git add is done"),
            }
        }
    }
}