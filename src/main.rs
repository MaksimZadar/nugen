mod author;
mod nugen_commands;
mod version;
use author::handle_author_commands;
use clap::Parser;
use nugen_commands::{Cli, Commands};
use version::handle_version_commands;

fn main() {
    let args = Cli::parse();

    match &args.command {
        Some(Commands::Version { command }) => handle_version_commands(command),
        Some(Commands::Author { command }) => handle_author_commands(command),
        None => {}
    }
}
