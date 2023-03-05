/// A representation of the Squid parser.
pub struct Parser;

impl Parser {
    pub fn run(source: String) {
        // Split source code by line endings
        let lines = source.split("\n");

        for line in lines {
            // Split by parentheses
            let mut calls = line.split("(");

            match calls.next() {
                Some("push") => match calls.next() {
                    Some(text) => {
                        println!("{}", text.split(")").next().unwrap());
                    }
                    None => {
                        println!("Missing parameter in function `push`");
                    }
                },
                _ => {}
            }
        }
    }
}
