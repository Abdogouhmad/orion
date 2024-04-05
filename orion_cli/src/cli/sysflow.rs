use crate::Sys;
use commandcrafter::{color::Col, execute::Execute};
use inquire::MultiSelect;
use orion_lib::arch::{simplecmd, update};
use std::{
    env, fs,
    process::{self},
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
        if args.disk {
            simplecmd::LinuxCmd::the_duf();
        }
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
                            "pacman" => update::arch_update("pacman"),
                            "yay" => update::arch_update("yay"),
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
}
