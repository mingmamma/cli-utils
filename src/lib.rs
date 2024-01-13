//! This is a library that provides utilities for command-line tools.
//! So far it only provides a function to read a line from stdin.
//! # Examples:
//! ```
//! use cli_utils::read_stdin;
//! let input = read_stdin();
//! ```

use std::{io::{self, BufRead}, error::Error};
use clap::Parser;
use colors::Color;

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
fn read_stdin() -> Result<String, std::io::Error> {
    let stdin = std::io::stdin();
    let mut reader = io::BufReader::new(stdin.lock());
    _read_stdin(&mut reader)
}

fn _read_stdin<R: BufRead>(reader: &mut R) -> Result<String, std::io::Error> {
    let mut line = String::new();
    reader
        .read_line(&mut line)?;
    
    Ok(line.trim().to_string())
}

fn split(s: String, delimiter: &String, field: usize) -> Result<String, &str> {
    let s_parts: Vec<&str> = s.split(delimiter).collect();
    match s_parts.get(field) {
        Some(&s) => Ok(s.to_string()),
        None => Err("No field found at index")
    }
}

fn display(s: String, color: &String) -> Result<(), &str> {
    let color = match_color(color)?;

    let mut color_struct = colors::ColorString {
        color: color,
        string: s,
        colorised: String::new(),
    };

    color_struct.paint();

    println!("{}", color_struct.colorised);

    Ok(())
}

fn match_color(color: &String) -> Result<Color, &str> {
    match color.as_str() {
        "r" => Ok(colors::Color::Red),
        "b" => Ok(colors::Color::Blue),
        "y" => Ok(colors::Color::Yellow),
        "g" => Ok(colors::Color::Green),
        _ => Err("Unmatched color input"),
    }
}

pub fn run(cli: &Cli) -> Result<(), Box<dyn Error>> {
    let input_s = read_stdin()?;

    let output_s = split(input_s, &cli.delimiter, cli.field)?;

    display(output_s, &cli.color);

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