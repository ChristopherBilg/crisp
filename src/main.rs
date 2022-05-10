mod atom;
mod lexical_analyzer;
mod program_arguments;
mod syntax_parser;

use clap::Parser;
use log::{debug, error, info};
use std::io::{self, BufRead, Write};

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
    let program_args = program_arguments::ProgramArguments::parse();
    info!("Successfully parsed program arguments.");

    // Interactive mode
    if program_args.interactive {
        handle_interactive_mode();
        std::process::exit(0);
    }

    // Command-line mode
    match program_args.command_line {
        Some(_) => {
            handle_command_line_mode();
            std::process::exit(0);
        }
        None => debug!("Command-line mode not used."),
    }

    // File-input mode
    match program_args.filename {
        Some(_) => {
            handle_file_input_mode();
            std::process::exit(0);
        }
        None => debug!("File input mode not used."),
    }

    // If no modes given, then default to interactive mode
    handle_interactive_mode();
    std::process::exit(0);
}

pub fn handle_interactive_mode() {
    let mut line;
    let stdin = io::stdin();

    loop {
        line = String::new();

        print!("crisp => ");
        io::stdout().flush().unwrap();
        stdin
            .lock()
            .read_line(&mut line)
            .expect("Unable to read line from 'stdin'.");

        let tokens =
            lexical_analyzer::tokenize(&line).expect("Unable to tokenize the given input.");

        for token in tokens {
            println!("{}", token);
        }
    }
}

pub fn handle_command_line_mode() {}

pub fn handle_file_input_mode() {}
