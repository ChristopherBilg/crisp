mod cli;

use clap::Parser;
use log::{error, info};

fn main() {
    // Initialize the logger utility
    match env_logger::try_init() {
        Ok(_) => info!("Successfully initialized logger."),
        Err(error) => {
            error!("Failed to initialize logger.");
            error!("{}", error.to_string());
            std::process::exit(1);
        }
    };

    // Parse the derived program arguments (CLI)
    let program_args = cli::program_arguments::ProgramArguments::parse();
    info!("Successfully parsed program arguments.");

    if program_args.interactive {
        println!("Interactive");
    }
}
