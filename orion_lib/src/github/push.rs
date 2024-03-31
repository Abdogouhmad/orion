use commandcrafter::{color::Col, execute::Execute};
use inquire::{InquireError, Select, Text};

pub fn apply_push() {
    let variety_commits = vec![
        "New Improvement to the code base 🚀",
        "Working on new feature 👷‍♂️",
        "Bug is Fix 🐛",
        "Docs are updated 📚",
        "Code is styled 🫠",
        "Codebase is refactored 🏭",
        "Test is updated 🤖",
        "Other changes🙂",
        "Customized Commit 😎",
    ];
    // select option
    let selected_commit = Select::new("Select a commit type", variety_commits).prompt();

    match selected_commit {
        Ok(commit_type) => match commit_type {
            "Customized Commit 😎" => {
                let customize_commit = Text::new("Please Enter Commit Messege 😎:").prompt();
                push_changes(&customize_commit)
            }
            _ => push_changes(&Ok(commit_type.to_string())),
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn push_changes(commit: &Result<String, InquireError>) {
    match commit {
        Ok(commit) => {
            // track the changes :)
            let add_result = Execute::run("git", &["add", "."]);
            if let Err(err) = add_result {
                eprintln!("Error adding changes: {:?}", err);
                std::process::exit(1);
            }

            // commit the changes
            let commit_result = Execute::run("git", &["commit", "-m", &commit]);
            if let Err(err) = commit_result {
                eprintln!("Error committing changes: {:?}", err);
                std::process::exit(1);
            }

            // get the branch the name :/
            let branch_result = Execute::run("git", &["rev-parse", "--abbrev-ref", "HEAD"]);
            let branch_name = match branch_result {
                Ok(bytes) => String::from_utf8_lossy(&bytes).trim().to_string(),
                Err(err) => {
                    eprintln!("Error getting branch name: {:?}", err);
                    std::process::exit(1);
                }
            };

            // push the branch head name :')
            let push_result =
                Execute::run("git", &["push", "--set-upstream", "origin", &branch_name]);
            if push_result.is_err() {
                eprintln!("Error pushing changes");
                std::process::exit(1);
            }
            println!("{}", Col::print_col(&Col::Magenta, "Code is pushed"));
        }
        Err(e) => println!("{e}"),
    }
}
