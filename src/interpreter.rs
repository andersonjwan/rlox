use std::fs;
use std::io;
use std::io::{BufRead, BufReader, Write};

use super::Result;

pub fn open(filename: String) -> Result<()> {
    let source = fs::read_to_string(filename)?;
    interpret(source)?;

    Ok(())
}

pub fn interactive() -> Result<()> {
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
            interpret(line)?;
        }
    }

    Ok(())
}

fn interpret(source: String) -> Result<()> {
    if !source.trim().is_empty() {
        println!("interpreting... \"{}\"", source.trim());
    }

    Ok(())
}
