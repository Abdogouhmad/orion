use std::process::exit;

use commandcrafter::{color::Col, execute::Execute, filestore::Filestore};

pub fn arch_update(name: &str) {
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
