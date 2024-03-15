use crate::Sys;
use inquire::{
    autocompletion::{Autocomplete, Replacement},
    Text,
};
use std::collections::HashSet;

#[derive(Clone, Default)]
pub struct FileCreate;

impl FileCreate {
    pub fn python(args: &Sys) {
        // Handle the python command
        if args.py {
            let name = Text::new("What is your name?")
                .with_autocomplete(FileCreate)
                .prompt();

            match name {
                Ok(name) => {
                    if name == "RUST" {
                        println!("welcome to rust")
                    } else if name == "PYTHON" {
                        println!("welcome to python")
                    } else {
                        println!("Hello {}", name)
                    }
                }
                Err(_) => println!("An error occurred when asking for your name, try again later."),
            }
        }
    }
}

impl Autocomplete for FileCreate {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, inquire::CustomUserError> {
        let mut suggestions = HashSet::new();
        suggestions.insert("python".to_string().to_uppercase());
        suggestions.insert("Rust".to_string().to_uppercase());
        // Add Python as a suggestion
        // You can add more languages here

        // Filter suggestions based on input
        let filtered_suggestions: Vec<String> = suggestions
            .iter()
            .filter(|s| s.starts_with(input))
            .cloned()
            .collect();

        Ok(filtered_suggestions)
    }
    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<inquire::autocompletion::Replacement, inquire::CustomUserError> {
        // If a suggestion is highlighted, return it as the completion
        if let Some(suggestion) = highlighted_suggestion {
            Ok(Replacement::Some(suggestion))
        } else {
            // If no suggestion is highlighted, return None
            Ok(Replacement::None)
        }
    }
}
