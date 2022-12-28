extern crate core;

mod cmd;

use crate::cmd::lib::Format;
use anyhow::Result;
use clap::{command, Parser, Subcommand};
use cmd::app::App;
use std::process::exit;
use std::time::Duration;

#[derive(Parser)]
#[command(
    name = "zzk",
    author = "fanyang89",
    version = "0.1.0",
    about = "Yet another ZooKeeper cli",
    arg_required_else_help = true,
    next_line_help = true
)]
pub struct Cli {
    #[arg(
        long,
        short,
        value_enum,
        default_value = "table",
        help = "Output format",
        global = true
    )]
    format: Format,

    #[arg(
        long,
        short,
        default_value = "127.0.0.1:2181",
        env = "ZOO_HOSTS",
        help = "ZooKeeper servers that connects to",
        global = true
    )]
    zoo_hosts: String,

    #[arg(
        long,
        short,
        default_value_t = 3000,
        help = "Connection timeout",
        global = true
    )]
    timeout: u64,

    #[arg(
        long,
        short,
        default_value_t = false,
        help = "Enable quiet mode",
        global = true
    )]
    quiet: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

impl Cli {
    pub fn get_zoo_hosts(&self) -> Vec<String> {
        self.zoo_hosts
            .split(",")
            .map(|v| v.trim().to_string())
            .collect()
    }

    pub fn get_timeout(&self) -> Duration {
        Duration::from_millis(self.timeout)
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Get value
    #[command(arg_required_else_help = true)]
    Get {
        key: String,

        #[arg(default_value_t = false, long, short, help = "Follow value changes")]
        watch: bool,
    },
    /// List children
    #[command(arg_required_else_help = true)]
    List {
        key: String,

        #[arg(default_value_t = false, long, short)]
        recursive: bool,

        #[arg(default_value_t = false, long, short)]
        show_value: bool,
    },
    /// Set value
    #[command(arg_required_else_help = true)]
    Set { key: String, value: String },
    /// Checks key exists or not
    #[command(arg_required_else_help = true)]
    Exists { key: String },
    /// Delete key
    #[command(arg_required_else_help = true)]
    Delete { key: String },
    /// Show current roles: Follower, Leader or Standalone
    #[command(arg_required_else_help = true)]
    Role {},
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();
    let app = App::new(&cli);
    let rc: Result<String> = match &cli.command {
        Some(Commands::Get { key, watch }) => app.get(key, watch),
        Some(Commands::List {
            key,
            recursive,
            show_value,
        }) => app.list(key, recursive, show_value),
        Some(Commands::Role {}) => app.get_role(),
        Some(Commands::Set { key, value }) => app.set(key, value),
        Some(Commands::Exists { key }) => app.exists(key),
        Some(Commands::Delete { key }) => app.delete(key),
        _ => unreachable!(),
    };

    if cli.quiet {
        match rc {
            Ok(_) => exit(0),
            Err(_) => exit(1),
        }
    } else {
        match rc {
            Ok(out) => println!("{}", out),
            Err(err) => println!("Error, {}", err),
        }
    }
}
