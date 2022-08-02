mod config;
mod interpreter;

pub use config::Config; // re-export

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn run(config: Config) -> Result<()> {
    match config.filename {
        Some(f) => interpreter::open(f),
        None => interpreter::interactive(),
    }
}
