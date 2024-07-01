use anyhow::Context;
use ecosystem::MyError;
use std::fs;

fn main() -> Result<(), anyhow::Error> {
    print!("Hello world");
    let filename = "foo.txt";
    fs::File::open(filename).with_context(|| format!("Failed to open {}", filename))?;

    fail_with_error().context("Failed to fail with error")?;
    Ok(())
}

fn fail_with_error() -> Result<(), MyError> {
    Err(MyError::Custom("This is a custom error".to_string()))
}
