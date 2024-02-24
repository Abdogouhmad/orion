#![allow(unused_imports)]
#![allow(dead_code)]

use clap::Parser;
use commandcrafter::{color::Col, execute::Execute};

/// Whisper CLI tool meant to minimize the amount of written command line in the terminal.
#[derive(Parser, Debug)]
#[command(version = "1.0.0", about, long_about)]
pub struct Sys {
    /// List pacman or yay packages that need update
    #[arg(short, long, default_value = "")]
    list: String,

    /// update the package either pacman, yay or both
    #[arg(
        short,
        long,
        default_value = "",
        long_help = "this command option give the ability to choose the package manager to operate either pacman, yay or both of them"
    )]
    update: String,
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
        if args.list == "pacman" {
            let p = Execute::run("pacman", &["-Qu", "--color=always"]);
            Execute::print_into_console(&p);
        } else if args.list == "yay" {
            let y = Execute::run("yay", &["-Qu", "--color=always"]);
            Execute::print_into_console(&y);
        } else if args.list == "both" {
            let p = Execute::run("pacman", &["-Qu", "--color=always"]);
            let y = Execute::run("yay", &["-Qu", "--color=always"]);
            let c = &[p, y].concat();
            Execute::print_into_console(&c);
        }

        // update option command
        if args.update == "pacman" {
            println!("pacman is updating wait please...");
        } else if args.update == "yay" {
            println!("yay is updating wait please...")
        } else if args.update == "both" {
            println!("yay and pacman are updating wait please...")
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
    }
}