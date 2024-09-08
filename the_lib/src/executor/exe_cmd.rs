use anyhow::{Context, Result};
use std::env;
use std::process::Stdio;
use std::{io::Error, path::Path};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command as UnixCmd;
use tokio::task;
use tokio::{
    self,
    fs::{self, File},
    sync::mpsc,
};

pub struct Execute;

impl Execute {
    /// Runs the command and displays the output directly to stdout/stderr (without logging)
    pub async fn run(command: &str, command_args: &[&str]) -> Result<(), Error> {
        // Spawn a tokio task for executing the command.
        let command = command.to_string();
        let command_args: Vec<String> = command_args.iter().map(|&s| s.to_string()).collect();

        let handle = task::spawn(async move {
            let output = UnixCmd::new(&command)
                .args(&command_args)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .await;

            match output {
                Ok(status) if status.success() => {
                    let exit_code = status.code().unwrap_or(-1);
                    std::process::exit(exit_code);
                }
                Ok(status) => {
                    let exit_code = status.code().unwrap_or(-1);
                    std::process::exit(exit_code);
                }
                Err(e) => {
                    eprintln!("Failed to execute command '{}': {}", command, e);
                }
            }
        });
        // Wait for the spawned task to finish
        if let Err(e) = handle.await {
            eprintln!("Task panicked: {}", e);
        }

        Ok(())
    }

    /// Executes a command, reads its output asynchronously, and logs the output to a file in /logs/ folder
    pub async fn execmd(command: &str, command_args: &[&str]) -> Result<(), Error> {
        // Ensure the /logs/ directory exists
        let path_comb = format!("{}/Desktop/logs", env::var("HOME").unwrap());
        let log_dir = Path::new(&path_comb);
        if !log_dir.exists() {
            fs::create_dir_all(log_dir).await?;
        }

        // Set the log file path using the command name
        let log_file_path = log_dir.join(format!("{}.log", command));
        let mut log_file = File::create(&log_file_path).await?;

        // channel for rcv and produce
        let (tx, mut rx) = mpsc::channel(32);
        let command = command.to_string();
        let command_args: Vec<String> = command_args.iter().map(|&s| s.to_string()).collect();

        // Spawn a task to execute the command
        task::spawn(async move {
            // create a child process command
            let mut child = UnixCmd::new(&command)
                .args(&command_args)
                .stdout(Stdio::piped())
                .spawn()
                .map_err(|e| format!("Failed to execute command '{}': {}", command, e))
                .unwrap();

            // Capture the child's stdout
            if let Some(stdout) = child.stdout.take() {
                let mut reader = BufReader::new(stdout).lines();

                // Send each line to the channel
                while let Some(line) = reader.next_line().await.unwrap_or(None) {
                    if tx.send(line.into_bytes()).await.is_err() {
                        break;
                    }
                }
            }

            // Wait for the command to complete
            let _status = child
                .wait()
                .await
                .context("Failed to wait for child process");
        });

        // Receive and log the output
        while let Some(bytes) = rx.recv().await {
            // Convert bytes to string for logging
            let output = String::from_utf8_lossy(&bytes);

            // Print output to stdout (optional)
            println!("{}", output);

            // Write the output to the log file
            log_file.write_all(output.as_bytes()).await?;
            log_file.write_all(b"\n").await?; // Add newline
        }

        Ok(())
    }
}
