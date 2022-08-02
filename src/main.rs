use std::env;
use std::process;

use rlox::Config;

fn main() {
    let config = Config::new(env::args());

    if let Err(e) = rlox::run(config) {
        eprintln!("rlox: Error: {}", e);
        process::exit(64);
    }
}
