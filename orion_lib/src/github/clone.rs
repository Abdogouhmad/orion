use commandcrafter::{color::Col, execute::Execute};
use inquire::Text;

pub fn apply_clone() {
    let username = Text::new("Enter the owner of repo: ").prompt();
    let repo = Text::new("Enter the name of the repo: ").prompt();
    let depth = Text::new("Enter the depth of cloning: ")
        .with_default("0")
        .prompt_skippable();

    if let (Ok(username), Ok(repo), Ok(depth)) = (username, repo, depth) {
        let clonefmt = format!("git@github.com:{}/{}.git", username, repo);
        let mut clone_pattern = vec!["clone"];

        // Only add depth option if depth is provided and greater than 0
        if let Some(depth) = depth.as_deref() {
            let depth_int: usize = depth.parse().unwrap_or(0);
            if depth_int > 0 {
                clone_pattern.push("--depth");
                clone_pattern.push(depth);
            }
        }

        clone_pattern.push(&clonefmt);

        let res = Execute::run("git", &clone_pattern);
        if res.is_ok() {
            println!("{}", Col::Green.print_col("Clone the repo well"))
        } else if res.is_err() {
            println!("{}", Col::Red.print_col("Clone the repo didn't go well"))
        }
    }
}
