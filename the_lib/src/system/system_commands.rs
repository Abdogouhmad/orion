use std::path::Path;
use std::time::Duration;
use std::{env, fs};

use crate::{colprintln, executor::Execute};
use crate::{eclprintln, Orn};
use anyhow::{Error, Result};
use tokio::task;
use tokio::time::sleep;

pub struct SystemFlow;

impl SystemFlow {
    pub async fn handle_sys_cmds(args: &Orn) -> Result<(), Error> {
        if args.list {
            let argscmd = ["-Qu", "--color=always"];
            Execute::run("pacman", &argscmd).await?;
            Execute::run("yay", &argscmd).await?;
        }

        if args.delete {
            Self::handle_log_deletion().await.unwrap();
        }

        if args.storage {
            // check if duf is installed
            let duf = Path::new("/usr/bin/duf");
            if !Path::exists(duf) {
                eprintln!("go download duf: https://github.com/muesli/duf");
            }

            // execute the command
            Execute::run("duf", &["--all"]).await?;
        }

        if let Some(update_pkgs) = &args.update {
            for pkg in update_pkgs {
                let cmd_args = ["-Syu", "--noconfirm"];
                let pacman = ["pacman", "-Syu", "--noconfirm"];
                if pkg.contains("yay") {
                    Execute::execmd(pkg, &cmd_args).await?;
                } else if pkg.contains("pacman") {
                    Execute::execmd("sudo", &pacman).await?;
                } else {
                    eclprintln!("<r>{pkg}</> is not supported yet");
                };
            }
        }

        Ok(())
    }

    async fn handle_log_deletion() -> Result<(), std::io::Error> {
        let handler = task::spawn(async move {
            colprintln!("<y>Deleting the log folder... \n</>");
            // sleep the task
            sleep(Duration::from_secs(3)).await;
            // get the path of the logs
            let log_path = env::var("HOME").unwrap() + "/Desktop/logs";

            // remove the directory
            match fs::remove_dir_all(log_path) {
                Ok(_) => {
                    colprintln!("<g>Log folder removed well done! </>");
                }
                Err(e) => {
                    eprintln!("\x1b[1;31mSomething went wrong:\x1b[0m {e}");
                }
            }
        });

        handler.await.unwrap();
        Ok(())
    }
}
