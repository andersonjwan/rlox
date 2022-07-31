mod config;
mod interpreter;

pub use config::Config; // re-export
use std::error;

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    match config.filename {
        Some(f) => interpreter::open(f),
        None => interpreter::interactive(),
    }
}
