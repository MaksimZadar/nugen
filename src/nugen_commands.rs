use clap::{Parser, Subcommand};

use crate::author::AuthorSubCommands;
use crate::version::VersionSubCommands;

#[derive(Parser, Debug)]
#[command(name = "Nugen")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manipulate the <version> tag of a Nuspec file
    Version {
        #[command(subcommand)]
        command: VersionSubCommands,
    },
    /// Manipulate the <author> tag of a Nuspec file
    Author {
        #[command(subcommand)]
        command: AuthorSubCommands,
    },
}
