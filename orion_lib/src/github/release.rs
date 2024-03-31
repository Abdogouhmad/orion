use inquire::{Confirm, Text};

pub fn apply_release() {
    let tag = Text::new("Enter the version of your app 🚀: ").prompt();
    let msg = Text::new("Message for the tag 😉: ").prompt();
    let confirm_push = Confirm::new("Do you want to push the tag ❓🤔: ")
        .with_default(false)
        .prompt();
    if let (Ok(t), Ok(m), Ok(pt)) = (tag, msg, confirm_push) {
        println!("{}; {}", t, m);
        if pt {
            println!("from lib ok")
        } else if !pt {
            println!("ok bye")
        }
        // let msg_fmt = format!("\"{}\"", m);
        // let res = Execute::run("git", &["tag", "-a", &t, "-m", &msg_fmt]);
        // if res.is_ok() {
        //     println!("{}", Col::Green.print_col("Tag created successfully"))
        // } else if res.is_err() {
        //     println!("{}", Col::Red.print_col("Tag creation failed"))
        // }
    }
}
