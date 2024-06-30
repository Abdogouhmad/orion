use commandcrafter::{color::Col, execute::Execute};
use std::{env, fs, path::Path, process};

pub enum LinuxCmd {
    Duf,
}

impl LinuxCmd {
    // # deleting()
    // deleting is function that deletes logs folder within the desktop folder
    // logs contains many op such yay and pacman logs.
    pub fn deleting() {
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

    /// # the_duf()
    /// is a command that shows the disk usage of your hard drive
    /// with argument of all which will list all the partions have been used in your hard disk
    pub fn the_duf() {
        let duf = Path::new("/usr/bin/duf");
        if !Path::exists(duf) {
            eprintln!("go download duf: https://github.com/muesli/duf");
        }
        let rst = Execute::exe("duf", &["--all"]);
        if rst.is_err() {
            eprintln!("something went wrong")
        }
    }
}
