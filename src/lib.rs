use core::str;
use itertools::Itertools;
use std::{
    env::args,
    io::{self, Read},
};

const CRAB: &str = r###"
        \
         \
           _~^~^~_
       \) /  o o  \ (/
         '_   u   _'
         \ '-----' /
"###;

const DEFAULT_WIDTH: usize = 80;

pub fn get_input() -> Result<Vec<String>, io::Error> {
    let mut lines: Vec<String> = args()
        .skip(1)
        .collect::<String>()
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();
    if lines.is_empty() {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        lines = input
            .split('\n')
            .map(String::from)
            .filter(|s| !s.is_empty())
            .collect();
    }
    Ok(lines)
}

pub fn get_terminal_width() -> usize {
    let (width, _) = term_size::dimensions().unwrap_or((DEFAULT_WIDTH, 0));
    width
}

pub fn is_valid_width(width: usize) -> bool {
    let min_width = CRAB.lines().map(|line| line.len()).max().unwrap();
    width >= min_width
}

fn cut_line(line: &str, width: usize) -> Vec<String> {
    if line.len() <= width {
        return vec![line.to_string()];
    }

    let mut cut_lines = Vec::new();
    let mut cur_line = String::new();

    'word_loop: for word in line.split_whitespace() {
        if !cur_line.is_empty() {
            cur_line += " ";
        }
        if cur_line.len() == width {
            cut_lines.push(cur_line.trim().to_string());
            cur_line = "".to_string();
        }

        if !cur_line.is_empty() && (cur_line.len() == width || cur_line.len() + word.len() > width)
        {
            cut_lines.push(cur_line.trim().to_string());
            cur_line = "".to_string();
        }
        if cur_line.is_empty() && word.len() > width {
            let word_lines = cut_word(word, width);
            let total = word_lines.len();
            for (i, l) in word_lines.into_iter().enumerate() {
                if i == total - 1 {
                    cur_line = l;
                    continue 'word_loop;
                }
                cut_lines.push(l);
            }
        }
        cur_line += word;
    }
    if !cur_line.is_empty() {
        cut_lines.push(cur_line);
    }

    cut_lines
}

fn cut_word(word: &str, width: usize) -> Vec<String> {
    if word.len() <= width {
        return vec![word.to_string()];
    }

    word.chars()
        .chunks(width)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .collect()
}

fn format_final_line(line: &str, begin: &str, end: &str, width: usize) -> String {
    let padding = " ".repeat(width - line.len() + 1);
    format!("{} {}{}{}", begin, line, padding, end)
}

pub fn make_boxed_text(input: Vec<String>, term_width: usize) -> String {
    if input.is_empty() {
        return "".to_string();
    }
    let extra_space = 4; // 'c ... c'
    let width = term_width - extra_space;

    let all_lines: Vec<_> = input.iter().flat_map(|l| cut_line(l, width)).collect();

    let max_line_len = all_lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let cap = "-".repeat(max_line_len + 2);
    let mut boxed_text = format!(" {} ", cap);

    match all_lines.len() {
        1 => {
            let box_line = format_final_line(&all_lines[0], "<", ">", max_line_len);
            boxed_text = format!("{}\n{}", boxed_text, box_line);
        }
        2 => {
            let first = format_final_line(&all_lines[0], "/", "\\", max_line_len);
            let second = format_final_line(&all_lines[1], "\\", "/", max_line_len);
            boxed_text = format!("{}\n{}\n{}", boxed_text, first, second);
        }
        _ => {
            for line in all_lines {
                let box_line = format_final_line(&line, "(", ")", max_line_len);
                boxed_text = format!("{}\n{}", boxed_text, box_line);
            }
        }
    }
    boxed_text = format!("{}\n {} {}", boxed_text, cap, CRAB);
    boxed_text
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;
    //#[test]
    //fn test_make_boxed_text() {
    //    let expected = format!(
    //        " --------------- \n< Hello, World! >\n --------------- {}",
    //        CRAB
    //    );
    //    assert_eq!(make_boxed_text("Hello, World!"), expected);
    //}
    #[rstest]
    #[case::normal(vec!["aaaa", "aaaa", "aa"], 4)]
    #[case::exact(vec!["aaaa"], 4)]
    #[case::smaller(vec!["aaa"], 4)]
    #[case::multiple_of_width(vec!["aaaa", "aaaa"], 4)]
    #[case::greater_than_width(vec!["aaaa", "aa"], 4)]
    #[case::special_characters(vec!["aa@#@", "@!$%"], 5)]
    #[case::unicode_characters(vec!["你好世", "界"], 3)]
    #[case::empty(vec![""], 4)]
    fn test_cut_word(#[case] expected_base: Vec<&str>, #[case] width: usize) {
        let expected: Vec<String> = expected_base.into_iter().map(|w| w.to_string()).collect();
        let to_cut: &str = &expected.clone().into_iter().collect::<String>();
        let cut = cut_word(to_cut, width);

        assert_eq!(cut, expected);
    }

    #[rstest]
    #[case::normal(vec!["This is", "a test", "string"], 7, " ")]
    #[case::exact(vec!["This is", "a test"], 7, " ")]
    #[case::smaller(vec!["This is a", "test"], 9, " ")]
    #[case::special_characters(vec!["aa@#@@!$", "% $$"], 8, " ")]
    #[case::unicode_characters(vec!["你好世界", "こんにちは"], 5, " ")]
    #[case::empty(vec![""], 4, " ")]
    #[case::long_word(vec!["aaaa", "aaaa", "aa"], 4, "")]
    fn test_cut_line(
        #[case] expected_base: Vec<&str>,
        #[case] width: usize,
        #[case] join_on: &str,
    ) {
        let expected: Vec<String> = expected_base.into_iter().map(|w| w.to_string()).collect();
        let to_cut: &str = &expected.clone().join(join_on);
        let cut = cut_line(to_cut, width);

        assert_eq!(cut, expected);
    }

    #[test]
    fn test_cut_line_exotic() {
        let line = "aaa aaaaa aa";
        let expected: Vec<String> = vec!["aaa", "aaaa", "a aa"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let cut = cut_line(line, 4);
        assert_eq!(cut, expected);
    }
}
