mod atom;
mod environment;
mod evaluator;
mod lexical_analyzer;
mod program_arguments;
mod syntax_parser;

use clap::Parser;
use log::{debug, error, info};
use std::{
    cell::RefCell,
    io::{self, BufRead, Write},
    rc::Rc,
};

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
            handle_command_line_mode(program_args.command_line.unwrap());
            std::process::exit(0);
        }
        None => debug!("Command-line mode not used."),
    }

    // If no modes given, then default to interactive mode
    handle_interactive_mode();
    std::process::exit(0);
}

pub fn handle_interactive_mode() {
    let mut line;
    let stdin = io::stdin();
    let mut environment = Rc::new(RefCell::new(environment::Environment::new()));

    loop {
        line = String::new();

        print!("crisp => ");
        io::stdout().flush().unwrap();
        stdin
            .lock()
            .read_line(&mut line)
            .expect("Unable to read line from 'stdin'.");

        let value = evaluator::evaluate(&line, &mut environment).unwrap();
        match value {
            atom::Atom::Void => {}
            atom::Atom::Integer(n) => println!("{}", n),
            atom::Atom::Bool(b) => println!("{}", b),
            atom::Atom::Symbol(s) => println!("{}", s),
            atom::Atom::Lambda(params, body) => {
                println!("Lambda(");
                for param in params {
                    println!("{} ", param);
                }
                println!(")");
                for expr in body {
                    println!(" {}", expr);
                }
            }
            _ => println!("{}", value),
        }
    }
}

pub fn handle_command_line_mode(string: String) {
    let mut environment = Rc::new(RefCell::new(environment::Environment::new()));

    let value = evaluator::evaluate(&string, &mut environment).unwrap();
    match value {
        atom::Atom::Void => {}
        atom::Atom::Integer(n) => println!("{}", n),
        atom::Atom::Bool(b) => println!("{}", b),
        atom::Atom::Symbol(s) => println!("{}", s),
        atom::Atom::Lambda(params, body) => {
            println!("Lambda(");
            for param in params {
                println!("{} ", param);
            }
            println!(")");
            for expr in body {
                println!(" {}", expr);
            }
        }
        _ => println!("{}", value),
    }
}
