use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Init {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    Build {
        #[arg(default_value = ".")]
        source: PathBuf,
        
        /// The output directory
        #[arg(default_value = "./dist")]
        output: PathBuf,
    },
    /// Serve the documentation locally
    Serve {
        /// The directory to serve
        #[arg(default_value = "./dist")]
        path: PathBuf,
        
        /// The port to serve on
        #[arg(default_value_t = 3000)]
        port: u16,
    },
}

fn main() {
    let args = Args::parse();
    match args.command {
        Command::Init { path } => {
            println!("Initializing project at {:?}", path);
        }
        Command::Build { source, output } => {
            println!("Building project from {:?} to {:?}", source, output);
        }
        Command::Serve { path, port } => {
            println!("Serving documentation from {:?} on port {}", path, port);
        }
    }
}
