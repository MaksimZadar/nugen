mod author;
mod file_loader;
mod nugen_commands;
mod version;
use author::handle_author_commands;
use clap::Parser;
use file_loader::FileOperationErrors;
use nugen_commands::{Cli, Commands};
use version::handle_version_commands;

fn main() -> Result<(), Option<FileOperationErrors>> {
    let args = Cli::parse();

    match &args.command {
        Some(Commands::Version { command }) => Ok(handle_version_commands(command)?),
        Some(Commands::Author { command }) => Ok(handle_author_commands(command)),
        None => Err(None),
    }
}
