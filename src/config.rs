pub struct Config {
    pub filename: Option<String>,
}

impl Config {
    pub fn new<T: Iterator<Item = String>>(mut args: T) -> Config {
        args.next(); // ignore executable path
        Config {
            filename: args.next(),
        }
    }
}
