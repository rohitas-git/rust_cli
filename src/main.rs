// grrs is a small tool that searches for a string in a file.

#![allow(unused)]

use std::rt::panic_count::get_count;

use anyhow::{Context, Result};
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

fn main() ->Result<()> {

    // argument from CLI; type Cli {pattern,path}
    let args = Cli::parse();

    let content = get_count(&args)

    // Finding the matches of the pattern in the content.
    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());
    
    Ok(());
}







/*
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

let pattern = std::env::args().nth(1).expect("no pattern given");
let path = std::env::args().nth(2).expect("no path given");

let args = Cli {
    pattern: pattern,
    path: std::path::PathBuf::from(path),
};

This works, but it’s not very convenient. 
How would you deal with the requirement to support --pattern="foo" or --pattern "foo"? 
How would you implement --help?

*/