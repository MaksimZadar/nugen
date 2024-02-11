use clap::Subcommand;

use crate::file_loader::{
    get_nuspec_info, open_nuspec_file, save_nuspec_file, FileOperationErrors, NuspecInfo,
};

#[derive(Subcommand, Debug)]
pub enum VersionSubCommands {
    /// Sets the <version> tag to specified value.
    /// Usage: nugen version set xx.xx.xx
    Set {
        /// Set the Nuspec version to the given value
        version_number: String,

        /// By default will look in CWD for a Nuspec,
        /// but you can specify a custom path with this flag
        #[arg(short, long)]
        file: Option<String>,
    },
}

pub fn handle_version_commands(command: &VersionSubCommands) -> Result<(), FileOperationErrors> {
    match command {
        VersionSubCommands::Set {
            version_number,
            file,
        } => handle_set(version_number, file),
    }
}

fn handle_set(
    version_number: &String,
    file_path: &Option<String>,
) -> Result<(), FileOperationErrors> {
    let mut file = match open_nuspec_file(&file_path) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut nuspec_info = match get_nuspec_info(&mut file) {
        Ok(i) => i,
        Err(e) => return Err(e),
    };

    nuspec_info.metadata.version = version_number.clone();

    let nuspec_file: NuspecInfo = NuspecInfo {
        info: nuspec_info,
        file,
    };

    match save_nuspec_file(nuspec_file) {
        Ok(_) => {}
        Err(e) => return Err(e),
    };

    return Ok(());
}
