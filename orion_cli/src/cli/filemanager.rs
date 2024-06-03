#![allow(unused_imports)]
use crate::Sys;
use commandcrafter::execute::Execute;
use inquire::{Select, Text};
use std::process::exit;
// use std::collections::HashSet;

#[derive(Clone, Default)]
pub struct FileCreate;

impl FileCreate {
    pub fn create_project(args: &Sys) {
        if args.file {
            // select a language
            let languages = vec!["RUST", "PYTHON"];
            let c = Select::new("Pick a language 😄", languages).prompt();
            match c {
                Ok(lang) => {
                    // match the language
                    if lang == "RUST" {
                        // RUST
                        self::FileCreate::rust_project();
                    } else if lang == "PYTHON" {
                        // PYTHON
                        self::FileCreate::python_project();
                    }
                }
                Err(_) => {
                    println!("Something went wrong in selection");
                    exit(1)
                }
            }
        }
    }

    /// create a rust project based on the user input
    fn rust_project() {
        let type_prj = vec!["--lib", "--bin"];
        let tp = Select::new("Pick type of project", type_prj).prompt();
        match tp {
            Ok(ty_name) => {
                let name_of_project = Text::new("What is the name of project?").prompt();
                match name_of_project {
                    Ok(name) => {
                        // let cargo_flags = ["--vcs", "none", &ty_name];
                        let _ = Execute::run("cargo", &["new", &name, "--vcs", "none", ty_name]);
                    }
                    Err(_) => eprintln!("smthg went wrong"),
                }
            }
            Err(_) => eprintln!("smth went wrong"),
        }
    }

    // TODO: still thinking about the python is it single file / env project
    fn python_project() {
        println!("you are in py now");
    }
}
