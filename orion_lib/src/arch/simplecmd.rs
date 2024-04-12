// use commandcrafter::{color::Col, execute::Execute, filestore::Filestore};
use commandcrafter::execute::Execute;
// use crossterm::style::Print;
// use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

// TODO: create commands like duf for disk size
// TODO: check if the command exists in path /usr/bin/duf
// TODO: not then provide github link for duf telling to install
// TODO: du for measuring the size of files and folders within the range of current dir
// if any error print it

pub enum LinuxCmd {
    Duf,
}

impl LinuxCmd {
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
