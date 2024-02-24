#![allow(unused_imports)]
use std::path::PathBuf;

use clap::{error, Parser}; // ! unused imports
use commandcrafter::{color::Col, execute::Execute};
/// whispercli is a cli tool that manage your system update based on your Linux distribution
#[derive(Parser, Debug)]
#[command(version = "1.0.0", about, long_about)]
struct Args {
    /// list the files that need to be updated
    #[arg(short, long)]
    list: bool,

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

fn main() {
    let args = Args::parse();

    if args.list {
        let f = Execute::run("ls", &["-l"]);
        Execute::print_into_console(&f);
    } else if args.wight {
        let w = Execute::run("du", &["-h", "--max-depth=1", "."]);
        Execute::print_into_console(&w);
    } else if args.delete {
        let log = std::env::var("HOME").unwrap() + "/Desktop/log";
        let r = std::fs::remove_dir_all(&log);
        match r {
            Ok(()) => {
                println!(
                    "{}",
                    Col::print_col(&Col::Green, "log folder deleted successfully")
                );
                std::process::exit(0)
            }
            Err(e) => {
                println!(
                    "{}: {}",
                    Col::print_col(&Col::Red, "can't delete log folder"),
                    e
                );
                std::process::exit(1)
            }
        }
    }
}
