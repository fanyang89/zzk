extern crate core;

mod cmd;

use crate::cmd::lib::Format;
use anyhow::Result;
use clap::{command, Parser, Subcommand};
use cmd::app::App;
use std::time::Duration;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(long, short, value_enum, default_value = "table")]
    format: Format,

    #[arg(long, short, default_value = "127.0.0.1:2181", env = "ZOO_HOSTS")]
    zoo_hosts: String,

    #[arg(long, short, default_value_t = 3000)]
    timeout: u64,

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
    Get {
        key: String,

        #[arg(default_value_t = false, long, short)]
        watch: bool,
    },
    List {
        key: String,

        #[arg(default_value_t = false, long, short)]
        recursive: bool,

        #[arg(default_value_t = false, long, short)]
        show_value: bool,
    },
    Set {
        key: String,
        value: String,

        #[arg(default_value_t = false, long, short)]
        watch: bool,
    },
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
        _ => unreachable!(),
    };

    match rc {
        Ok(out) => println!("{}", out),
        Err(err) => println!("Error, {}", err),
    }
}
