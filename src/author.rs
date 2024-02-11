use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum AuthorSubCommands {
    /// Sets the <version> tag to specified value.
    /// Usage: nugen version set xx.xx.xx
    Set { author_name: String },
}

pub fn handle_author_commands(command: &AuthorSubCommands) {
    match command {
        AuthorSubCommands::Set { author_name } => {}
    }
}
