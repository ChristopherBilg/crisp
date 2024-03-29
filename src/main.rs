mod atom;
mod environment;
mod evaluator;
mod lexical_analyzer;
mod program_arguments;
mod syntax_parser;

use clap::Parser;
use std::{
    cell::RefCell,
    io::{self, BufRead, Write},
    rc::Rc,
};

fn main() {
    // Parse the derived program arguments (CLI)
    let program_args = program_arguments::ProgramArguments::parse();

    // Interactive mode
    if program_args.interactive {
        handle_interactive_mode();
    }

    std::process::exit(0);
}

fn handle_interactive_mode() {
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

        let evaluated_result = evaluator::evaluate(&line, &mut environment);
        match evaluated_result {
            Ok(atom) => println!("       > {}", atom),
            Err(error) => println!("       > {}", error),
        }
    }
}
