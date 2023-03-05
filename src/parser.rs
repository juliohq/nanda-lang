use std::collections::HashMap;

/// A representation of the Nanda parser.
pub struct Parser;

impl Parser {
    #[inline]
    pub fn run(source: String) {
        // Static source code analysis
        // Replace parenthesis
        let mut is_string = false; // Whether there's an open string
        let mut replace_line: usize = 1;
        let mut last_string: usize = 0; // Line of the last open string

        let binding: String = source
            .chars()
            .map(|char| {
                if char == '\n' {
                    // Increase replace line counter
                    replace_line += 1;
                }

                if is_string {
                    // Close string
                    if char == '\'' || char == '\"' {
                        is_string = false;
                    }

                    // Just return the character
                    char
                } else {
                    // Open string
                    if char == '\'' || char == '\"' {
                        last_string = replace_line;
                        is_string = true;
                    }

                    char
                }
            })
            .collect();

        // Make sure all strings were closed
        if is_string {
            println!("String not enclosed. (line {})", last_string);
            return;
        }

        // Split source code by line endings and filter single comment lines
        let lines = binding
            .split("\n")
            .map(|line| if line.starts_with("//") { "\n" } else { line });

        // Variable pool
        let mut string_pool: HashMap<&str, &str> = HashMap::new();

        // Line counter
        let mut counter = 1usize;

        // Run source code
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
                            // Split arguments
                            Some(text) => {
                                if text.contains("\'") {
                                    // TODO: Unify argument parsing
                                    match text.split("\'").skip(1).next() {
                                        Some(arg) => {
                                            println!("{}", arg.split(")").next().unwrap());
                                        }
                                        None => {
                                            println!(
                                                "Missing `text` parameter in function `push`. (line {})",
                                                counter
                                            );
                                        }
                                    }
                                } else if text.contains("\"") {
                                    // TODO: Unify argument parsing
                                    match text.split("\"").skip(1).next() {
                                        Some(arg) => {
                                            println!("{}", arg.split(")").next().unwrap());
                                        }
                                        None => {
                                            println!(
                                                "Missing `text` parameter in function `push`. (line {})",
                                                counter
                                            );
                                        }
                                    }
                                }
                            }
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
