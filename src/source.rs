pub struct Source;

use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};

use crate::Parser;

impl Source {
    #[inline]
    pub fn interpret<P>(path: P)
    where
        P: AsRef<Path> + Display,
    {
        match fs::read_to_string(&path) {
            Ok(source) => {
                // Pass source code to parser
                Parser::run(source, PathBuf::from(path.as_ref()));
            }
            Err(e) => {
                println!("Error reading `{}` file. ({})", path, e);
            }
        }
    }
}
