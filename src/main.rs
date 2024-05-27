use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::exit;
use clap::Parser;
use serde::{Deserialize, Serialize};

/// Simple program to extract the required fields for instancesync
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// the input .json, commonly minecraftinstance.json
    #[arg(short, long)]
    input: PathBuf,

    /// the output file, must be .json
    #[arg(short, long)]
    output: PathBuf,
}

fn main() {
    let args = Args::parse();

    println!("opening file");

    let mut input = File::open(args.input).unwrap_or_else(|err| {
        eprintln!("failed opening file: {}", err.to_string());
        exit(1);
    });

    let mut contents = Default::default();

    println!("reading file contents");
    input.read_to_string(&mut contents);

    println!("parsing file contents");
    let instance: Instance = serde_json::from_str(&contents).unwrap_or_else(|err| {
        eprintln!("failed parting file contents: {}", err.to_string());
        exit(1);
    });

    let output = serde_json::to_string_pretty(&instance).unwrap_or_else(|err| {
        eprintln!("failed serializing output: {}", err.to_string());
        exit(1);
    });


    println!("writing to output file");
    // create and or wipe file
    std::fs::write(&args.output, output).unwrap_or_else(|err| {
        eprintln!("failed writing to output file: {}", err.to_string());
        exit(1);
    });

}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Instance {
    #[serde(rename = "installedAddons")]
    installed_addons: Vec<Addon>,
    #[serde(rename = "cachedScans")]
    cached_scans: Vec<Scan>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Addon {
    #[serde(rename = "installedFile")]
    installed_file: AddonFile,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AddonFile {
    #[serde(rename = "fileNameOnDisk")]
    file_name_on_disk: String,
    #[serde(rename = "downloadUrl")]
    download_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Scan {
    #[serde(rename = "folderName")]
    folder_name: String
}