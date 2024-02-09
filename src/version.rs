use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum VersionSubCommands {
    /// Sets the <version> tag to specified value.
    /// Usage: nugen version set xx.xx.xx
    Set { version_number: String },
}

pub fn handle_version_commands(command: &VersionSubCommands) {
    match command {
        VersionSubCommands::Set { version_number } => handle_set(version_number),
    }
}

fn handle_set(version_number: &String) {
    println!("{}", version_number);
}
