#[cfg(test)]
mod tests {
    use std::{io, process::Command};

    fn binary() -> Command {
        Command::new("target/release/squid")
    }

    #[test]
    fn hello() {
        // Run code
        let output = binary()
            .arg("src/tests/hello.sq")
            .output()
            .expect("Failed test");

        // Assert
        assert_eq!(output.stdout.as_slice(), b"Hello World!\n");
    }
}
