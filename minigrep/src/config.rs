//! # Minigrep's Configuration struct and related error
//!
//! See the [module-level documentation](index.html) for more

use std::path::PathBuf;
use std::error::Error;
use std::fmt;
use std::env;

use unicase::UniCase;

pub use ConfigError::*;

/// Struct with program configuration
///
/// This struct contains the query string, the path
/// to the file to be searched and if the search
/// should be case sensitive or not.
///
/// While initialising, it is decided to be case
/// insensitive if there is an environment
/// variable `CASE_INSENSITIVE` with value different
/// from `"0"` or `"false"`. Alternatively, there is
/// a flag `-i` or `--case-insensitive` that can
/// be passed to the arguments to overrides this.
///
/// The last two arguments are consecutively the
/// query string and the file target.
#[derive(Debug)]
pub struct Config {
    /// the query string
    pub query: String,
    /// path to target file
    pub filename: PathBuf,
    /// if search should be case sensitive
    pub case_sensitive: bool
}

impl PartialEq for Config {
    fn eq(&self, other: &Self) -> bool {
        self.case_sensitive == other.case_sensitive
            && self.filename == other.filename
            && if self.case_sensitive {
                self.query == other.query
            } else {
                type Query<'a> = UniCase<&'a str>;

                Query::from(&self.query) == Query::from(&other.query)
            }
    }
}

impl Config {
    /// Builds the configuration from an [`Iterator`]
    /// with the arguments. Any argument starting with
    /// `'-'` is considered a flag.
    ///
    /// Might result in [`Err`] with [`NotEnoughArguments`]
    /// or [`UnknownFlag`].
    ///
    /// # Example
    ///
    /// ```
    /// use minigrep::Config;
    /// use minigrep::config::ConfigError::*;
    /// use std::iter;
    ///
    /// assert_eq!(Config::new(iter::empty()), Err(NotEnoughArguments))
    /// ```
    pub fn new(mut args: impl Iterator<Item=String>) -> Result<Self, ConfigError> {
        let mut case_sensitive = match env::var("CASE_INSENSITIVE") {
            Err(_) => true,
            Ok(value) => value == "0" || value.to_lowercase() == "false"
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

    /// Builds the configuration with borrowed [`str`]
    /// by cloning contents to a [`String`].
    ///
    /// # Example
    ///
    /// ```
    /// use minigrep::Config;
    /// use minigrep::config::ConfigError::*;
    ///
    /// let args = "--unknown-flag word file.txt".split_whitespace();
    /// let flag = String::from("--unknown-flag");
    ///
    /// assert_eq!(Config::from_borrowed(args), Err(UnknownFlag(flag)))
    /// ```
    ///
    /// *See [`Config::new`] for details*
    pub fn from_borrowed<T: AsRef<str>>(args: impl Iterator<Item=T>) -> Result<Self, ConfigError> {
        Self::new(&mut args.map(|s| String::from(s.as_ref())))
    }

    /// Builds the configuration using [`std::env::args`].
    ///
    /// # Example
    ///
    /// ```
    /// use minigrep::Config;
    ///
    /// match Config::from_args() {
    ///     Ok(config) => println!("CONFIGURATION: {:?}", config),
    ///     Err(err) => println!("ERROR: {}", err)
    /// }
    /// ```
    ///
    /// *See [`Config::new`] for details*
    pub fn from_args() -> Result<Self, ConfigError> {
        Self::new(env::args().skip(1))
    }
}


/// Possible errors for building [`Config`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    /// Represents an unknown flag passed as
    /// argument, containing said flag
    UnknownFlag(String),
    /// Indicates that there aren't enough
    /// arguments to build configuration
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
