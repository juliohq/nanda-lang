use std::collections::HashMap;

/// A representation of the Nanda parser.
pub struct Parser;

impl Parser {
    #[inline]
    pub fn run(source: String) {
        // Split source code by line endings and filter empty lines
        let lines = source.split("\n");

        // Variable pool
        let mut string_pool: HashMap<&str, &str> = HashMap::new();

        // Line counter
        let mut counter = 1u64;

        for line in lines {
            // Split parts (a.k.a. [let, x, =, "hello"])
            let mut parts = line.split_whitespace();

            match parts.next() {
                Some("let") => {
                    match parts.next() {
                        // Check if data is missing previously
                        Some("=") => {
                            println!("Missing variable data in declaration. (line {})", counter);
                            break;
                        }
                        Some(name) => {
                            // Check if variable was already declared
                            match string_pool.get(name) {
                                Some(_) => {
                                    println!(
                                        "Variable `{}` already declared in the current scope. (line {})",
                                        name,
                                        counter,
                                    );
                                    break;
                                }
                                None => {
                                    // Check assignment operator
                                    match parts.next() {
                                        Some("=") => {
                                            // Check variable data
                                            match parts.next() {
                                                Some(data) => {
                                                    // Append data to heap
                                                    string_pool.insert(name, data);
                                                }
                                                None => {
                                                    println!("Missing variable data in declaration. (line {})", counter);
                                                    break;
                                                }
                                            }
                                        }
                                        _ => {
                                            println!("Missing `=` variable declaration operator. (line {})", counter);
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            println!(
                                "Missing identifier in variable declaration. (line {})",
                                counter
                            );
                        }
                    }
                }
                _ => {
                    // Split calls by parentheses
                    let mut calls = line.split("(");

                    match calls.next() {
                        // Skip newlines
                        Some("\n") | Some(" ") | Some("") => {}
                        // Print-like statement
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
                            println!("Unknown `{}` statement. (line {})", call, counter);
                            break;
                        }
                        None => {}
                    }
                }
            }

            // Increment line counter
            counter += 1;
        }
    }
}
