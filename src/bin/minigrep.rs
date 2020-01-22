use std::process;

use book::minigrep::{self, Config};


fn main() {
    let config = Config::from_args().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(&config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
