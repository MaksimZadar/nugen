use clap::{Parser, Subcommand};

use crate::{version::VersionSubCommands, AuthorSubCommands};

#[derive(Parser, Debug)]
#[command(name = "Nugen")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// By default will look in CWD for a Nuspec,
    /// but you can specify a custom path with this flag
    #[arg(long, short)]
    pub file: Option<String>,
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
