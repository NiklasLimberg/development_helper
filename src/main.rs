use std::path::PathBuf;

use clap::{Parser, Subcommand};


#[macro_use]
extern crate serde_derive;


#[path = "commands/prepare_pr.rs"]
mod prepare_pr;


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
    /// Creates a branch, a commit, and a changelog
    Create {
        /// Ticket id
        #[clap(short, long, action)]
        id: String,

        /// Ticket title
        #[clap(short, long, action)]
        title: String,
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

    match &cli.command {
        Some(Commands::Create { id, title }) => {
           prepare_pr::run(id, title)
        }
        None => {}
    }
}