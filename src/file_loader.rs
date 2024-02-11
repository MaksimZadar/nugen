use glob::glob;
use quick_xml::se::to_string;
use serde_derive::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    os::windows::fs::FileExt,
};

#[derive(Debug)]
pub struct NuspecInfo {
    pub info: Package,
    pub file: File,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case", rename = "package")]
pub struct Package {
    #[serde(rename = "@xmlns")]
    pub xmlns: String,
    pub metadata: Metadata,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Metadata {
    pub id: String,
    pub version: String,
    pub description: String,
    pub authors: String,
}

#[derive(Debug)]
pub enum FileOperationErrors {
    NoNuspecFound,
    FailedToOpenFile,
    FailedToReadFileToBuffer,
    FailedToDeserialize,
    FailedToSerialize,
    FailedToWriteToFile,
    CouldNotClearFile,
    FailedToRewindToStartOfFile,
}

pub fn get_nuspec_info(file: &mut File) -> Result<Package, FileOperationErrors> {
    let mut file_buff: String = String::new();

    match file.read_to_string(&mut file_buff) {
        Ok(_) => {}
        Err(_) => return Err(FileOperationErrors::FailedToReadFileToBuffer),
    };

    let package: Package = match quick_xml::de::from_str(&file_buff) {
        Ok(p) => p,
        Err(_) => return Err(FileOperationErrors::FailedToDeserialize),
    };

    return Ok(package);
}

pub fn open_nuspec_file(file_path: &Option<String>) -> Result<File, FileOperationErrors> {
    // Set default file path string
    let file_path = match &file_path {
        Some(path) => path,
        None => "./*.nuspec",
    };

    let mut paths = match glob(file_path) {
        Ok(p) => p,
        Err(_) => return Err(FileOperationErrors::NoNuspecFound),
    };

    // Get first file result
    let binding = paths.nth(0);
    let first_file_result = match &binding {
        Some(f) => f,
        None => return Err(FileOperationErrors::NoNuspecFound),
    };

    let first_file = match first_file_result {
        Ok(f) => f,
        Err(_) => return Err(FileOperationErrors::NoNuspecFound),
    };

    // Open the file
    let file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(first_file)
    {
        Ok(f) => f,
        Err(_) => return Err(FileOperationErrors::FailedToOpenFile),
    };

    return Ok(file);
}

pub fn save_nuspec_file(mut nuspec_file: NuspecInfo) -> Result<(), FileOperationErrors> {
    let new_content = match to_string(&nuspec_file.info) {
        Ok(c) => c,
        Err(_) => return Err(FileOperationErrors::FailedToSerialize),
    };

    match nuspec_file.file.set_len(38) {
        Ok(_) => {}
        Err(_) => return Err(FileOperationErrors::CouldNotClearFile),
    };

    match nuspec_file.file.seek(SeekFrom::End(0)) {
        Ok(_) => {}
        Err(_) => return Err(FileOperationErrors::FailedToRewindToStartOfFile),
    };

    match nuspec_file.file.write_all(new_content.as_bytes()) {
        Ok(_) => {}
        Err(_) => return Err(FileOperationErrors::FailedToWriteToFile),
    };

    println!("{}", new_content);
    return Ok(());
}
