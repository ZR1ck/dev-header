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
        let line = if field.key_visible {
            build_key_value_line(field, key_len, value_len)
        } else {
            build_line(field, max_len)
        };

        lines.push(line);

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

fn build_line(field: &Field, line_len: usize) -> String {
    match field.align {
        Alignment::Center => format!("{:^line_len$}", field.default_value),
        Alignment::Left => format!("{:<line_len$}", field.default_value),
    }
}

pub fn build_key_value_line(field: &Field, key_len: usize, value_len: usize) -> String {
    let mut result = String::new();
    let lines = wrap_text(&field.default_value, value_len.into());

    // println!("Lines wrapped: {:?}", lines);

    for i in 0..lines.len() {
        let line = {
            let key = if i == 0 {
                field.key.clone()
            } else {
                format!("{}", " ".repeat(key_len))
            };

            match field.align {
                Alignment::Center => {
                    format!(" {:^key_len$} : {:^value_len$}", key, field.default_value)
                }
                Alignment::Left => {
                    format!(" {:<key_len$} : {:<value_len$}", key, field.default_value)
                }
            }
        };
        result.push_str(&line);
    }
    result
}

pub fn wrap_text(text: &str, line_len: usize) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let mut line = String::from("");

    for word in text.split_whitespace() {
        if line.len() + word.len() + 1 <= line_len {
            line.push_str(word);
            line.push(' ');
        } else {
            result.push(line);
            line = word.to_string();
        }
    }
    result.push(line);
    result
}
