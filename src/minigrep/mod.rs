pub mod config;
pub mod search;

pub use config::Config;
pub use search::*;

use std::error::Error;
use std::fs;


pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let results: Box<dyn Iterator<Item=&str>> = if config.case_sensitive {
        Box::new(search(&config.query, &contents))
    } else {
        Box::new(search_case_insensitive(&config.query, &contents))
    };

    results.for_each(|line|
        println!("{}", line)
    );

    Ok(())
}
