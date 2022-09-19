use anyhow::{Context, Result, Error};
// Crate to handle CLI Arguments
// The most popular library for parsing command-line arguments is called clap
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for in the file
    pattern: String,

    /// The path to the file to be read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}
/// Finding the matches of the pattern in the content.
/// It takes a string slice, a string slice, and a mutable reference to a type that implements the Write
/// trait, and it writes all lines from the first string slice that contain the second string slice to
/// the writer
/// 
/// Arguments:
/// 
/// * `content`: &str - a reference to a string slice that holds the content of the file
/// * `pattern`: &str
/// * `writer`: impl std::io::Write
pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}


//content of the file
    /// - read_to_string() reads the contents of a file into a string.
    ///     - with_context() is a method that is defined on all Result<T, E> values. 
    ///     - It takes a closure that returns a string and returns a new Result<T, E> value. 
    ///     - If the original Result<T, E> value is an Ok(T), the new Result<T, E> value is also an
    /// Ok(T). 
    ///     - If the original Result<T, E> value is an Err(E), the new Result<T, E> value is an Err(E)
    /// with the error message from the closure.
    ///     - The ? operator is used to return from the function early if the Result<T, E> value is an
    /// Err(E).
fn get_content(&args: &Cli)-> Result<String, Error>{
    std::fs::read_to_string(&args.path)
    .with_context(|| format!("could not read file `{}`", args.path.display()))
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
    // (The b prefix makes this a byte string literal so its type is going to be &[u8] instead of &str).
}