use std::{
    error::Error,
    io::{self, Write},
};

use crabsay::{get_input, make_boxed_text};

fn main() -> Result<(), Box<dyn Error>> {
    let line = get_input()?;
    let boxed_text = make_boxed_text(&line);
    io::stdout().write_all(boxed_text.as_bytes())?;
    Ok(())
}
