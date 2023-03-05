#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::process::Command;

    fn binary() -> Command {
        if cfg!(debug_assertions) {
            if Path::new("target/debug/nanda").exists() {
                Command::new("target/debug/nanda")
            } else {
                panic!("You have to `cargo build` first!!!");
            }
        } else {
            if Path::new("target/release/nanda").exists() {
                Command::new("target/release/nanda")
            } else {
                panic!("You have to `cargo build --release` first!!!");
            }
        }
    }

    #[test]
    fn hello() {
        // Run code
        let output = binary().arg("src/tests/hello.nd").output().unwrap();

        // Assert
        assert_eq!(output.stdout.as_slice(), b"Hello World!\n");
    }
}
