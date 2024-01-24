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

// Note that the split function does NOT need to own the input String to be splitted
// and hence the input parameter s is taken as &str

// the split() function takes a lifetime parameter 'a, since the function takes a 
// reference in its parameter list, and this explicit annotation is required.
// The annotation entails that the lifetime of parameter s must be at least as long
// as function split(), or in other words, the lifetime of function split cannot exceed
// that of its parameter s

/// This function splits the input s by the given delimiter and tries to get and return
/// the splited parts as a vec of str slice
pub fn split_by_delimiter<'a>(s: &'a str, delimiter: &str) -> Vec<&'a str> {
    
    // Alternatives just based on whichever enhances readability
    // https://doc.rust-lang.org/std/iter/trait.FromIterator.html#examples
    Vec::from_iter(s.split(delimiter))
    
    // the useful bit in the turbofish annotation is what collection type
    // to be collected into, and the element type can be easily inferred 
    // s.t. it is not as relevant
    // s.split(delimiter).collect::<Vec<_>>()
}

// This function tries to return the str slice in the vector by the parameter field as index
// The return type is a Result where the Err may occur if the passed in parameter field is INVALID index
pub fn get_field(s_parts: Vec<&str>, field: usize) -> Result<&str, &str> {

    match s_parts.get(field) {
        Some(&s_part) => Ok(s_part),
        None => Err("No field found at index")
    }
}

/// This function creates colorString struct with the given str slice
/// s.t. the colorsied field can be printed out
pub fn display(s: &str, color: &String) -> Result<(), &'static str> {
    let color = match_color(color)?;

    let mut color_struct = colors::ColorString {
        color,
        string: s,
        colorised: String::new(),
    };

    color_struct.paint();

    println!("{}", color_struct.colorised);

    Ok(())
}

fn match_color(color: &String) -> Result<Color, &'static str> {
    match color.as_str() {
        "r" => Ok(colors::Color::Red),
        "b" => Ok(colors::Color::Blue),
        "y" => Ok(colors::Color::Yellow),
        "g" => Ok(colors::Color::Green),
        _ => Err("Unmatched color input"),
    }
}

pub fn run(cli: &Cli) -> Result<(), Box<dyn Error>> {
    // the lifetime of String input_s starts when it is returned as an owned String from the read_stdin() call
    // the lifetime of String input_s ends when the scope of run method is completed
    let input_s = read_stdin()?;

    let s_parts = split_by_delimiter(&input_s, &cli.delimiter);

    let output_s = get_field(s_parts, cli.field)?;

    display(output_s, &cli.color)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::{_read_stdin, split_by_delimiter, get_field};

    #[test]
    fn test_read_input() {
        let input = "Hello World\n";
        let mut cursor = Cursor::new(input);
        
        let output = _read_stdin(&mut cursor).unwrap();
        assert_eq!(output, "Hello World");
    }

    #[test]
    fn test_read_input_empty() {
        let input = "\n";
        let mut reader = Cursor::new(input);
        let output = _read_stdin(&mut reader).unwrap();
        assert_eq!(output, "");
    }

    #[test]
    fn test_split_by_delimiter() {
        let output = split_by_delimiter("abc,efg", ",");
        assert_eq!(output, vec!["abc", "efg"]);
    }

    #[test]
    fn test_get_field() {
        let ok_output = get_field(vec!["abc", "efg"], 0);
        assert_eq!(ok_output, Ok("abc"));
    }
}