use std::env;

mod parser;
mod source;
mod tests;

use parser::Parser;
use source::Source;

fn main() {
    let mut args = env::args();

    // Skip first argument (a.k.a. squid binary)
    args.next();

    if let Some(path) = args.next() {
        Source::interpret(path);
    } else {
        println!("No file path provided.");
    }
}
