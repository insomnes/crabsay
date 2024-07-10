use std::{env::args, io};

const CRAB: &str = r###"
        \
         \
           _~^~^~_
       \) /  o o  \ (/
         '_   u   _'
         \ '-----' /
"###;

const MAX_LENGTH: usize = 80 - 4;

pub fn get_input() -> Result<String, io::Error> {
    let mut line: String = args().skip(1).collect::<Vec<String>>().join(" ");
    if line.is_empty() {
        io::stdin().read_line(&mut line)?;
        line = line.trim().to_string();
    }
    if line.len() > MAX_LENGTH {
        line.truncate(MAX_LENGTH);
    }
    Ok(line)
}

fn make_line_box(line: &str) -> String {
    let boxed_line = format!("< {} >", line);
    boxed_line
}

pub fn make_boxed_text(line: &str) -> String {
    let boxed_line = make_line_box(line);
    let boxed_text = format!(
        " {} \n{}\n {} {}",
        "-".repeat(boxed_line.len() - 2),
        boxed_line,
        "-".repeat(boxed_line.len() - 2),
        CRAB
    );

    boxed_text
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_make_boxed_text() {
        let expected = format!(
            " --------------- \n< Hello, World! >\n --------------- {}",
            CRAB
        );
        assert_eq!(make_boxed_text("Hello, World!"), expected);
    }
}
