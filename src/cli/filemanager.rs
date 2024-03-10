pub struct MoreCommands;
use crate::Sys;
impl MoreCommands {
    pub fn handle_more_commands(args: &Sys) {
        // Handle the ping command
        if args.ping {
            println!("Pong! Your Whisper CLI is up and running.");
        }
    }
}
