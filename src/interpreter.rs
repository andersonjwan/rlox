mod tokens;

use core::fmt;
use std::error;
use std::fs;
use std::io;
use std::io::{BufRead, BufReader, Write};

use super::Result as BoxResult;

#[derive(Debug)]
struct InterpreterError;

impl error::Error for InterpreterError {}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to interpret")
    }
}

pub fn open(filename: String) -> BoxResult<()> {
    let source = fs::read_to_string(filename)?;
    interpret(source)?;

    Ok(())
}

pub fn interactive() -> BoxResult<()> {
    let mut reader = BufReader::new(io::stdin());

    loop {
        let mut line = String::new();

        print!(">> ");
        io::stdout().flush()?; // std::io::Error

        let nbytes = reader.read_line(&mut line)?; // std::io::Error

        if nbytes == 0 {
            println!();
            break;
        } else {
            match interpret(line) {
                Ok(()) => continue,
                Err(e) => eprintln!("response: {}", e),
            }
        }
    }

    Ok(())
}

fn interpret(source: String) -> Result<(), InterpreterError> {
    if !source.trim().is_empty() {
        println!("interpreting... \"{}\"", source.trim());
    }

    Ok(())
}
