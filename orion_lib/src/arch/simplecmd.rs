// use commandcrafter::{color::Col, execute::Execute, filestore::Filestore};
use std::path::Path;
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
        let duf = Path::new("/usr/bin/duff");
        if !Path::exists(duf) {
            eprintln!("well well");
        } else {
            println!("u have it");
        }
        // todo!()
    }
}
