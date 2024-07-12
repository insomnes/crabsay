use std::{
    error::Error,
    io::{self, Write},
    process::exit,
};

use crabsay::{get_input, get_total_width, is_valid_width, make_boxed_text};

fn main() -> Result<(), Box<dyn Error>> {
    let width = get_total_width();
    if !is_valid_width(width) {
        eprintln!("Terminal too narrow to display crab");
        exit(1);
    }
    let lines = get_input()?;
    if lines.is_empty() {
        eprintln!("No input provided");
        exit(1);
    }
    let boxed_text = make_boxed_text(lines, width);
    io::stdout().write_all(boxed_text.as_bytes())?;
    Ok(())
}
