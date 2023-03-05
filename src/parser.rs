use std::{collections::HashMap, path::PathBuf};

/// A representation of the Nanda parser.
pub struct Parser;

impl Parser {
    #[inline]
    pub fn run(source: String, path: PathBuf) {
        // Static source code analysis
        // Replace parenthesis
        let mut is_string = false; // Whether there's an open string
        let mut string_enclosing: Option<char> = None;
        let mut replace_line: usize = 1;
        let mut last_string: usize = 0; // Line of the last open string
        let mut enclosing_error = false; // Whether an unexpected enclosing character was used

        // `map_while` is used here so it terminates the mapping immediately after an unexpected character error is raised
        let binding: String = source
            .chars()
            .map_while(|char| {
                if char == '\n' {
                    // Increase replace line counter
                    replace_line += 1;
                }

                if is_string {
                    // Close string
                    if char == '\'' || char == '\"' {
                        if string_enclosing.unwrap() != char {
                            // Set enclosing flag and terminate `map` as soon as possible
                            enclosing_error = true;
                            println!(
                                "String enclosed with unexpected character. (line {})",
                                replace_line
                            );
                            return None;
                        }

                        is_string = false;
                    }

                    // Just return the character
                    Some(char)
                } else {
                    // Open string
                    if char == '\'' || char == '\"' {
                        is_string = true;
                        string_enclosing = if char == '\'' { Some('\'') } else { Some('\"') };
                        last_string = replace_line;
                    }

                    Some(char)
                }
            })
            .collect();

        // Terminate program due to string enclosing error
        if enclosing_error {
            return;
        }

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
                Some("use") => {
                    match parts.next() {
                        Some(name) => {
                            // Include the module in the current execution
                            let module_path = path.with_file_name(format!("{}.nd", name));

                            if module_path.is_file() {
                                // TODO: Add module support
                            } else {
                                println!("Module `{}` not found (line {})", name, counter);
                                println!("{:?}", module_path);
                                break;
                            }
                        }
                        _ => {
                            println!("Missing module name (line {})", counter);
                            break;
                        }
                    }
                }
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
                    // Split calls by parenthesis
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
