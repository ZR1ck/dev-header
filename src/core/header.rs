use arboard::Clipboard;
use dialoguer::{Input, theme::ColorfulTheme};

use crate::core::template::{Alignment, Field, Template};

pub fn border_input(promt: &str, input_theme: &ColorfulTheme) -> Option<char> {
    let value: String = Input::with_theme(input_theme)
        .with_prompt(promt)
        .allow_empty(true)
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.is_empty() || input.chars().count() == 1 {
                Ok(())
            } else {
                Err("Please enter exactly one character or leave empty")
            }
        })
        .interact_text()
        .unwrap();
    if value.is_empty() {
        None
    } else {
        Some(value.chars().next().unwrap())
    }
}

pub fn get_header(template: &Template) {
    let header = build_header(template);
    println!("\n{}\n", header);

    // copy to clipboard
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(header).unwrap();
}

pub fn build_header(template: &Template) -> String {
    // calculate key & value size
    let key_len = {
        let mut max = 0;
        for field in &template.fields {
            if field.key_visible && max < field.key.len() {
                max = field.key.len();
            }
        }
        max
    };
    let max_len = (template.length - 2) as usize; // exclude left, right border
    let value_len = max_len - key_len - 4; // space before key: 1     space after key: 3 (' : ')

    // println!("key_len: {}", key_len);
    // println!("value_len: {}", value_len);
    // println!("max_len: {}", max_len);

    // build header body
    let mut lines: Vec<String> = vec![];
    for field in &template.fields {
        let vec_lines = if field.key_visible {
            build_key_value_lines(field, key_len, value_len)
        } else {
            build_line(field, max_len)
        };

        for line in vec_lines {
            lines.push(line);
        }

        // println!("|{}|", line);
        // println!("size: {}", line.len());
    }
    // add space
    let empty_line = format!("{}", " ".repeat(max_len));
    for _ in 0..template.space_after {
        lines.push(empty_line.clone());
    }
    for _ in 0..template.space_before {
        lines.insert(0, empty_line.clone());
    }

    // add borders
    if let Some(border) = &template.border {
        // left
        if let Some(left) = border.left {
            for line in &mut lines {
                line.insert(0, left);
            }
        }

        // right
        if let Some(right) = border.right {
            for line in &mut lines {
                line.insert(line.len(), right);
            }
        }

        // top
        if let Some(top) = border.top {
            lines.insert(0, format!("/*{}", String::from(top).repeat(max_len)));
        } else {
            lines.insert(0, format!("/*"));
        }

        // bottom
        if let Some(bottom) = border.bottom {
            lines.push(format!("{}*/", String::from(bottom).repeat(max_len)));
        } else {
            lines.push(format!("*/"));
        }
    } else {
        for line in &mut lines {
            line.insert_str(0, "//");
        }
    }

    lines.join("\n")
}

fn build_line(field: &Field, line_len: usize) -> Vec<String> {
    let lines = wrap_text(&field.default_value, line_len);
    let mut result = vec![];
    for i in 0..lines.len() {
        let line = match field.align {
            Alignment::Center => format!("{:^line_len$}", lines[i]),
            Alignment::Left => format!("{:<line_len$}", lines[i]),
        };
        result.push(line);
    }
    result
}

pub fn build_key_value_lines(field: &Field, key_len: usize, value_len: usize) -> Vec<String> {
    let mut result = vec![];
    let lines = wrap_text(&field.default_value, value_len.into());

    // println!("Lines wrapped: {:?}", lines);

    for i in 0..lines.len() {
        let line = {
            let key = if i == 0 {
                format!(" {:<key_len$} : ", field.key.clone())
            } else {
                format!(" {:<key_len$}   ", " ".repeat(key_len))
            };

            match field.align {
                Alignment::Center => {
                    format!("{}{:^value_len$}", key, lines[i])
                }
                Alignment::Left => {
                    format!("{}{:<value_len$}", key, lines[i])
                }
            }
        };
        result.push(line);
    }
    result
}

pub fn wrap_text(text: &str, line_len: usize) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let mut line = String::from("");

    // println!("line_len: {line_len}");

    for word in text.split_whitespace() {
        // current word is longer than the remain space
        if word.len() > (line_len - line.len()) {
            // split to fit the remain space
            let remain = line_len - line.len();
            let s = &word[0..remain];
            line.push_str(s);
            result.push(line.clone());

            // number of lines for the remaining character
            let n = ((word.len() - remain) as f32 / line_len as f32).ceil() as usize;
            let mut start_pos = remain;
            let mut end_pos = std::cmp::min(start_pos + line_len, word.len());
            let mut i = 0;
            loop {
                let s = &word[start_pos..end_pos];
                line = s.to_string();
                if i >= n - 1 {
                    break;
                }
                result.push(s.to_string());
                start_pos = end_pos;
                end_pos = std::cmp::min(start_pos + line_len, word.len());
                i += 1;
            }
        }
        // current word is shorter
        else if line.len() + word.len() + 1 <= line_len {
            line.push_str(word);
            line.push(' ');
        }
        // line complete
        else {
            result.push(line);
            line = word.to_string();
        }
    }
    result.push(line);
    result
}
