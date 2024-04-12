use commandcrafter::{color::Col, execute::Execute};
use std::{env, fs, path::Path, process};

// TODO: create commands like duf for disk size
// TODO: check if the command exists in path /usr/bin/duf
// TODO: not then provide github link for duf telling to install
// TODO: du for measuring the size of files and folders within the range of current dir
// if any error print it

pub enum LinuxCmd {
    Duf,
}

impl LinuxCmd {
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
    pub fn the_duf() {
        let duf = Path::new("/usr/bin/duf");
        if !Path::exists(duf) {
            eprintln!("go download duf: https://github.com/muesli/duf");
        } else {
            let rs = Execute::exe("duf", &["--all"]);
            // let rs = Command::new("duf")
            //     .arg("--all")
            //     .stdout(Stdio::inherit())
            //     .spawn();
            match rs {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Command can't be executed {}", e);
                    std::process::exit(1)
                }
            }
        }
        // todo!()
    }
}
