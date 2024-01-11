//! This is a library that provides utilities for command-line tools.
//! So far it only provides a function to read a line from stdin.
//! # Examples:
//! ```
//! use cli_utils::read_stdin;
//! let input = read_stdin();
//! ```

use std::{io::{self, BufRead}, error::Error};
use clap::Parser;

pub mod config;
pub mod colors;

#[derive(Parser)]
pub struct Cli {
    #[arg(short('f'))]
    field: usize,
    #[arg(short('d'))]
    delimiter: String,
    #[arg(short('c'))]
    color: String,
    #[arg(long)]
    debug: bool,
}

/// This function reads a line from stdin and returns it as a Result<String, std::io::Error>.
/// # Examples:
/// ```
/// use cli_utils::read_stdin;
/// let input = read_stdin().unwrap();
/// ```
pub fn read_stdin() -> Result<String, std::io::Error> {
    let stdin = std::io::stdin();
    let mut reader = io::BufReader::new(stdin.lock());
    _read_stdin(&mut reader)
}

fn _read_stdin<R: BufRead>(reader: &mut R) -> Result<String, std::io::Error> {
    let mut line = String::new();
    reader
        .read_line(&mut line)?;
        //.expect("Failed to read input line");
    
    Ok(line.trim().to_string())
}

pub fn split(s: String, cli: &Cli) -> Result<String, &str> {
    let s_parts: Vec<&str> = s.split(&cli.delimiter).collect();
    match s_parts.get(cli.field) {
        Some(&s) => Ok(s.to_string()),
        None => Err("No field found at index")
    }
}

pub fn display(s: String, cli: &Cli) {
    let color = match cli.color.as_str() {
        "r" => colors::Color::Red,
        "b" => colors::Color::Blue,
        "y" => colors::Color::Yellow,
        "g" => colors::Color::Green,
        &_ => panic!("Invalid color option")
    };

    let mut color_struct = colors::ColorString {
        color: color,
        string: s,
        colorised: String::new(),
    };

    color_struct.paint();

    println!("{}", color_struct.colorised)
}

pub fn run(cli: &Cli) -> Result<(), Box<dyn Error>> {
    let input_s = read_stdin()?;

    let output_s = split(input_s, cli)?;

    display(output_s, cli);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::_read_stdin;

    #[test]
    fn test_read_input() {
        let input = "Hello World\n";
        let expected_output = "Hello World";
        let mut cursor = Cursor::new(input);
        
        let output = _read_stdin(&mut cursor).unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_read_input_empty() {
        let input = "\n";
        let expected_output = "";
        let mut reader = Cursor::new(input);
        let output = _read_stdin(&mut reader).unwrap();
        assert_eq!(output, expected_output);
    }
}