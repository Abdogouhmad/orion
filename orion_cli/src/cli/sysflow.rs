use crate::Sys;
use commandcrafter::execute::Execute;
use inquire::MultiSelect;
use orion_lib::arch::{simplecmd::LinuxCmd, update};
pub struct Syscmd;

impl Syscmd {
    /// # system_flow
    /// this method intends to operate over many operations such:
    /// * `list`: list the packages that need to be updated within pacman and yay package manager
    /// * `update`: update the packages within pacman and yay both at once
    /// * `weight`: list the weight of each folder within the same directory
    /// * `delete`: delete the log folder which has the logs of the update operation
    pub fn system_flow(args: &Sys) {
        if args.disk {
            LinuxCmd::the_duf();
        }
        // list option command

        if args.list {
            // list the packages needs to be updated for both
            let _ = Execute::exe("pacman", &["-Qu", "--color=always"]);
            let _ = Execute::exe("yay", &["-Qu", "--color=always"]);
        }

        // update command
        if args.update {
            // vector str of options
            let packagemanger = vec!["pacman", "yay"];
            // map the vec options
            let packager = MultiSelect::new(
                "choose the package manager you want to update 😄 🆙",
                packagemanger,
            )
            .prompt();
            // match the options
            match packager {
                Ok(pckg) => {
                    for p in pckg {
                        match p {
                            // TODO: fun for updating
                            "pacman" => update::arch_update("pacman"),
                            "yay" => update::arch_update("yay"),
                            _ => eprintln!("out of range"),
                        }
                    }
                }
                Err(_) => eprintln!("error"),
            }
        }

        // weight option command
        if args.weight {
            let w = Execute::run("du", &["-h", "--max-depth=1", ".", "--time"]);
            Execute::print_into_console(w);
        }

        // delete option command
        if args.delete {
            LinuxCmd::deleting()
        }
    }
}
