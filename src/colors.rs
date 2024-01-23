//! Colorized output utilities for the terminal using ANSI escape codes.
//! # Examples:
//! ```
//! use cli_utils::colors::*;
//! println!("{}{}{}", red("Red"), green("Green"), blue("Blue"));
//! ```

/// Creating a new instance of ColorString struct.
/// # Examples:
/// ```
/// use cli_utils::colors::*;
/// let color_string = ColorString{color: Color::Red, string: "Red", colorised: String::new()};
/// ```
pub struct ColorString<'a> {
    pub color: Color,
    pub string: &'a str,
    pub colorised: String,
}

pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

/// Returns a string with the ANSI escape code for red.
/// # Examples:
/// ```
/// use cli_utils::colors::*;
/// println!("{}", red("Red"));
/// ```
pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

pub fn green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}

pub fn yellow(s: &str) -> String {
    format!("\x1b[33m{}\x1b[0m", s)
}

pub fn blue(s: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", s)
}

pub fn reset(s: &str) -> String {
    format!("\x1b[0m{}\x1b[0m", s)
}

impl ColorString< '_> {
    pub fn paint(&mut self) {
        match self.color { 
            Color::Blue => self.colorised = blue(&self.string),
            Color::Green => self.colorised = green(&self.string),
            Color::Red => self.colorised = red(&self.string),
            Color::Yellow => self.colorised = yellow(&self.string),    
        }
    }

    pub fn reset(&mut self) {
        self.colorised = reset(&self.string)
    }

}