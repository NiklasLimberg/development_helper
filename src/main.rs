use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[macro_use]
extern crate serde_derive;

#[path = "commands/setup.rs"]
mod setup;

#[path = "commands/prepare_pr.rs"]
mod prepare_pr;

#[path = "commands/open_changelog.rs"]
mod open_changelog;

#[derive(Parser)]
#[clap(author, version, about, long_about = None,  )]
#[clap(arg_required_else_help = true)]
struct Cli {
    /// Sets a custom config file
    #[clap(short, long, value_parser, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[clap(short, long, action)]
    debug: bool,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Changes the config of the tool
    Setup {},

    /// Creates a branch, a commit, and a changelog
    Create {
        /// Ticket id
        #[clap(short, long, action)]
        id: String,

        /// Ticket title
        #[clap(short, long, action)]
        title: String,
    },

    EditChangelog {
        /// Ticket id
        #[clap(short, long, action)]
        id: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    if cli.debug {
        println!("Debug mode is on");
    }

    match cli.command {
        Some(Commands::Setup {}) => setup::run(),
        Some(Commands::Create { id, title }) => prepare_pr::run(id, title),
        Some(Commands::EditChangelog { id }) => open_changelog::run(id),
        None => {}
    }
}
