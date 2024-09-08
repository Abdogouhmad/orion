// import mods
mod commands;
mod compressor;
mod executor;
mod git;
mod macros;
mod system;
use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser,
};
use commands::OrnCommands;
use system::SystemFlow;

/// Orn is a rust based CLI tool that minimize the amount of redoing tasks.
#[derive(Parser, Debug)]
#[command(name = "orn",version = env!("CARGO_PKG_VERSION"),author = env!("CARGO_PKG_AUTHORS"), long_about, styles=Self::orn_style())]
pub struct Orn {
    /// List packages that needs to be updated
    #[arg(short, long)]
    list: bool,

    /// deleting file logs
    #[arg(short, long)]
    delete: bool,

    /// getting the disk usage
    #[arg(short, long)]
    storage: bool,

    /// Update Arch packages like pacman & yay
    /// Example: -u pacman yay or -u pacman
    #[arg(short, long)]
    update: Option<Vec<String>>,
    /// mores subcommands
    #[command(subcommand)]
    pub orncommand: Option<OrnCommands>,
}

impl Orn {
    #[tokio::main]
    pub async fn init_cli() {
        let args = Orn::parse();
        SystemFlow::handle_sys_cmds(&args).await.unwrap();
        OrnCommands::handle_subcommands(&args).await.unwrap();
    }
    pub fn orn_style() -> Styles {
        Styles::styled()
            .usage(AnsiColor::BrightRed.on_default())
            .header(AnsiColor::BrightGreen.on_default())
            .literal(AnsiColor::Cyan.on_default())
            .invalid(AnsiColor::Red.on_default())
            .error(AnsiColor::Red.on_default())
            .valid(AnsiColor::Green.on_default())
            .placeholder(AnsiColor::Green.on_default())
    }
}
