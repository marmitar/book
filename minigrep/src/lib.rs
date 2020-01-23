//! # Minigrep Utilities
//!
//! Configuration details in [`Config`]
//!
//! # Examples
//!
//! ## Running with explicit arguments
//!
//! ```
//! use minigrep::{Config, run};
//!
//! let args = "-i word file.txt";
//! let config = Config::from_borrowed(args.split_whitespace())
//!     .unwrap();
//!
//! if let Err(err) = run(&config) {
//!     println!("Runtime Error: {}", err)
//! }
//! ```
//!
//! ## Configuration from shell arguments
//!
//! ```
//! use minigrep::Config;
//! use minigrep::config::ConfigError;
//!
//! match Config::from_args() {
//!     Ok(config) => println!("Got the configuration!"),
//!     Err(err) => {
//!         println!("Oh no, this is quite unexpected!");
//!         println!("Error: {}", err)
//!     }
//! }
//! ```

extern crate unicase;
extern crate regex;

pub mod config;
pub mod search;

pub use config::Config;
pub use search::*;

use std::error::Error;
use std::fs;


/// Main function to run with specified [`Config`]
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let query = regex_builder(&config.query, config.case_sensitive)?;
    let results = search(query, &contents);

    results.for_each(|line|
        println!("{}", line)
    );

    Ok(())
}
