use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use std::io::{self, Write};

/// The main structure for CLI arguments
#[derive(Parser)]
#[command(name = "filehosting-cli")]
#[command(about = "A file hosting CLI application written in Rust", long_about = None)]
struct Cli {
    /// The command to run
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The subcommands for the CLI
#[derive(Subcommand)]
enum Commands {
    /// Upload a file
    Upload {
        /// Path to the file to upload
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    /// Download a file
    Download {
        /// Name of the file to download
        #[arg(value_name = "FILE_NAME")]
        file_name: String,
    },
    /// List all files
    List,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Upload { file }) => {
            println!("Uploading file: {:?}", file);

            // Check if the file exists
            if !file.exists() {
                println!("File does not exist.");
                return;
            }

            // Ensure the storage directory exists
            let storage_dir = PathBuf::from("files");
            if !storage_dir.exists() {
                fs::create_dir_all(&storage_dir).expect("Failed to create storage directory");
            }

            // Copy the file to the storage directory
            let dest = storage_dir.join(file.file_name().unwrap());
            fs::copy(&file, dest).expect("Failed to copy file");

            println!("File uploaded successfully!");
        }
        Some(Commands::Download { file_name }) => {
            println!("Downloading file: {}", file_name);

            let storage_dir = PathBuf::from("files");
            let file_path = storage_dir.join(file_name);

            if file_path.exists() {
                println!("File found: {:?}", file_path);
                // For now, just show the file path (future steps can handle actual downloading logic)
            } else {
                println!("File not found.");
            }
        }
        Some(Commands::List) => {
            println!("Listing all files:");

            let storage_dir = PathBuf::from("files");
            if storage_dir.exists() {
                let entries = fs::read_dir(storage_dir).expect("Failed to read directory");
                for entry in entries {
                    let entry = entry.expect("Failed to read entry");
                    println!("{}", entry.file_name().to_string_lossy());
                }
            } else {
                println!("No files found.");
            }
        }
        None => {
            println!("No command provided");
        }
    }
}
