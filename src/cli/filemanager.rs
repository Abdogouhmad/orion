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
            let c = Select::new("Pick a language", languages).prompt();
            match c {
                Ok(lang) => {
                    // match the language
                    if lang == "RUST" {
                        // choose between lib and bin
                        let type_prj = vec!["--lib", "--bin"];
                        let tp = Select::new("Pick type of project", type_prj).prompt();
                        match tp {
                            Ok(ty_name) => {
                                let name_of_project =
                                    Text::new("What is the name of project?").prompt();
                                match name_of_project {
                                    Ok(name) => {
                                        // let cargo_flags = ["--vcs", "none", &ty_name];
                                        let _ = Execute::run(
                                            "cargo",
                                            &["new", &name, "--vcs", "none", &ty_name],
                                        );
                                    }
                                    Err(_) => eprintln!("smthg went wrong"),
                                }
                            }
                            Err(_) => eprintln!("smth went wrong"),
                        }
                    } else if lang == "PYTHON" {
                        println!("you are in py now");
                    }
                }
                Err(_) => {
                    println!("Something went wrong in selection");
                    exit(1)
                }
            }
        }
    }
}

// impl Autocomplete for FileCreate {
//     fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, inquire::CustomUserError> {
//         let mut suggestions = HashSet::new();
//         suggestions.insert("python".to_string().to_uppercase());
//         suggestions.insert("Rust".to_string().to_uppercase());

//         // Filter suggestions based on input
//         let filtered_suggestions: Vec<String> = suggestions
//             .iter()
//             .filter(|s| s.starts_with(input))
//             .cloned()
//             .collect();

//         Ok(filtered_suggestions)
//     }
//     fn get_completion(
//         &mut self,
//         input: &str,
//         highlighted_suggestion: Option<String>,
//     ) -> Result<inquire::autocompletion::Replacement, inquire::CustomUserError> {
//         // If a suggestion is highlighted, return it as the completion
//         if let Some(suggestion) = highlighted_suggestion {
//             Ok(Replacement::Some(suggestion))
//         } else {
//             // If no suggestion is highlighted, return None
//             Ok(Replacement::None)
//         }
//     }
// }