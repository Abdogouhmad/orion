use commandcrafter::{color::Col, execute::Execute, filestore::Filestore};
use std::process::exit;

pub fn arch_update(name: &str) {
    let pac_flag = ["pacman", "-Syu", "--noconfirm"];
    let yay_flag = ["-Syu", "--noconfirm"];
    if name.contains("pacman") {
        // execute the cmd packman
        let pac = Execute::run("sudo", &pac_flag);
        // print the cmd output to console as clone output
        Execute::print_into_console(pac.clone());
        // get the output and store it as log file
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
        // execute the cmd yay
        let yay = Execute::run("yay", &yay_flag);
        // print the cmd output to console as clone output
        Execute::print_into_console(yay.clone());
        // get the output and store it as log file
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
    ];
}
