use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::{error, io};

pub fn open(filename: String) -> Result<(), Box<dyn error::Error>> {
    let source = fs::read_to_string(filename)?;
    interpret(source)?;

    Ok(())
}

pub fn interactive() -> Result<(), Box<dyn error::Error>> {
    let mut reader = BufReader::new(std::io::stdin());

    loop {
        let mut line = String::new();

        print!(">> ");
        io::stdout().flush()?;

        let nbytes = reader.read_line(&mut line)?;

        if nbytes == 0 {
            println!();
            break;
        } else {
            interpret(line)?;
        }
    }

    Ok(())
}

fn interpret(source: String) -> Result<(), Box<dyn error::Error>> {
    if !source.trim().is_empty() {
        println!("interpreting... \"{}\"", source.trim());
    }

    Ok(())
}
