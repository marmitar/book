//! # Minigrep regex builder and search function
//!
//! # Examples
//!
//! ## Regex building
//!
//! ```
//! use minigrep;
//!
//! let regex = minigrep::regex_builder("the", false).unwrap();
//!
//! assert!(regex.is_match("The"))
//! ```
//!
//! ## Invalid regex
//!
//! ```
//! use minigrep;
//!
//! let result = minigrep::regex_builder("~*!)", false);
//!
//! assert!(result.is_err())
//! ```
//!
//! ## Searching
//!
//! ```
//! use minigrep;
//!
//! let regex = minigrep::regex_builder("the", true).unwrap();
//! let mut iter = minigrep::search(regex, "The onion");
//!
//! assert_eq!(iter.next(), None)
//! ```
//!
//! See the [module-level documentation](index.html) for more

use regex::{Regex, RegexBuilder, Error};


/// Tries to build a [`Regex`] for search
pub fn regex_builder(query: &str, case_sensitive: bool) -> Result<Regex, Error> {
    let mut builder = RegexBuilder::new(query);
    builder.case_insensitive(! case_sensitive);

    builder.build()
}

/// Search for lines where regex match
pub fn search<'a>(regex: Regex, contents: &'a str) -> impl Iterator<Item=&'a str> {
    contents.lines()
        .filter(move |line| regex.is_match(line))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(regex_builder(query, true).unwrap(), contents).collect::<Vec<_>>()
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search(regex_builder(query, false).unwrap(), contents).collect::<Vec<_>>()
        );
    }
}
