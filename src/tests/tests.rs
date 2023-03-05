#[cfg(test)]
mod tests {
    use std::process::Command;

    fn binary() -> Command {
        if cfg!(debug_assertions) {
            Command::new("target/debug/nanda")
        } else {
            Command::new("target/release/nanda")
        }
    }

    #[test]
    fn hello() {
        // Run code
        let output = binary()
            .arg("src/tests/hello.nd")
            .output()
            .expect("Failed test");

        // Assert
        assert_eq!(output.stdout.as_slice(), b"Hello World!\n");
    }
}
