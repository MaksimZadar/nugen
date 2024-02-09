use std::{
    fs::{self, File, OpenOptions},
    io::{BufReader, Read},
};

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Package {
    metadata: Metadata,
}

#[derive(Deserialize, Debug)]
pub struct Metadata {
    id: String,
    version: String,
    description: String,
    authors: String,
}

const NS: &'static str = "http://schemas.microsoft.com/packaging/2010/07/nuspec.xsd";

pub fn read_file(file_path: &String) -> Package {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .expect("Could not open file!");

    let mut file_str: String = String::new();
    match file.read_to_string(&mut file_str) {
        Ok(_) => println!("Success"),
        Err(_) => println!("Fail"),
    };

    let package: Package =
        serde_xml_rs::from_str(file_str.as_mut_str()).expect("Something went wrong deserializing");

    return package;
}
