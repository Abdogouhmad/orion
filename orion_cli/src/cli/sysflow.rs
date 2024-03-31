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
