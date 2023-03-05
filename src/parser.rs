/// A representation of the Nanda parser.
pub struct Parser;

impl Parser {
    #[inline]
    pub fn run(source: String) {
        // Split source code by line endings and filter empty lines
        let lines = source.split("\n").filter(|line| !line.is_empty());

        // Line counter
        let mut counter = 1u64;

        for line in lines {
            // Split calls by parentheses
            let mut calls = line.split("(");

            match calls.next() {
                Some("push") => match calls.next() {
                    Some(text) => match text.split("\"").skip(1).next() {
                        Some(arg) => {
                            println!("{}", arg.split(")").next().unwrap());
                        }
                        None => {
                            println!(
                                "Missing `text` parameter in function `push`. (line {})",
                                counter
                            );
                        }
                    },
                    None => {
                        println!(
                            "Missing `text` parameter in function `push`. (line {})",
                            counter
                        );
                    }
                },
                Some(call) => {
                    println!("Unknown `{}` function call. (line {})", call, counter);
                    break;
                }
                None => {}
            }

            // Increment line counter
            counter += 1;
        }
    }
}
