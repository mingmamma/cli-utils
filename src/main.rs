use std::process;
use clap::Parser;
use cli_utils::{Cli, run};

fn main() {
    let commands = Cli::parse();
    
    if let Err(err) = run(&commands) {
        eprintln!("Application running error: {err}");
        process::exit(1);
    }
}