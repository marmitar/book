use std::path::PathBuf;
use std::error::Error;
use std::fmt;
use std::env;

pub use ConfigError::*;

pub struct Config {
    pub query: String,
    pub filename: PathBuf,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(mut args: impl Iterator<Item=String>) -> Result<Self, ConfigError> {
        let mut case_sensitive = match env::var("CASE_INSENSITIVE") {
            Err(_) => true,
            Ok(value) => value == "0" || value.to_lowercase() == "FALSE"
        };

        let query = loop {
            let arg = args.next()
                .ok_or(NotEnoughArguments)?;

            if ! arg.starts_with('-') {
                break arg;
            }

            match arg.as_ref() {
                "-i" | "--case-insensitive" => {
                    case_sensitive = false
                },
                _ => return Err(UnknownFlag(arg))
            }
        };

        let path = args.next()
            .ok_or(NotEnoughArguments)?;

        Ok(Config { query, filename: path.into(), case_sensitive })
    }

    pub fn from_borrowed<T: AsRef<str>>(args: impl Iterator<Item=T>) -> Result<Self, ConfigError> {
        Self::new(&mut args.map(|s| String::from(s.as_ref())))
    }

    pub fn from_args() -> Result<Self, ConfigError> {
        Self::new(env::args().skip(1))
    }
}


#[derive(Debug, Clone)]
pub enum ConfigError {
    UnknownFlag(String),
    NotEnoughArguments
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnknownFlag(flag) => write!(f, "unknown flag: '{}'", flag),
            NotEnoughArguments => write!(f, "not enough arguments")
        }
    }
}

impl Error for ConfigError { }
