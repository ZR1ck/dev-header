use arboard::Clipboard;
use clap::{ArgGroup, Parser, Subcommand};
use console::style;
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::{cmp, fs};

use crate::{
    config::Config,
    template::{Alignment, Border, Field, Template},
};

mod config;
mod template;

const TEMPLATE_FILE_PATH: &str = "template.json";

#[derive(Parser)]
#[command(
    name = "header",
    version,
    about = "CLI tool for code comment headers",
    author = "<duydangtr@gmail.com>"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new header
    New,
    /// List all headers
    List {
        /// Selection mode: select a header and copy it to clipboard
        #[arg(short, long)]
        selection: bool,
    },
    /// Get a header
    Get {
        /// Header name
        name: String,
        /// JSON view
        #[arg(short, long)]
        json: bool,
    },
    /// Get a header and apply placeholders
    Apply,
    #[command(group(ArgGroup::new("edit_options").required(true).args(&["border", "fields", "name", "spacing"])))]
    /// Edit an existing header
    Edit {
        /// Header name
        header_name: String,
        /// Edit header name
        #[arg(short, long)]
        name: Option<String>,
        /// Edit header spacing
        #[arg(short, long)]
        spacing: bool,
        /// Edit header border
        #[arg(short, long)]
        border: bool,
        /// Edit header fields
        #[arg(short, long)]
        fields: bool,
    },
    /// Delete a header
    Delete {
        #[arg(short, long, conflicts_with = "name")]
        all: bool,
        /// Header name
        #[arg(required_unless_present = "all")]
        name: Option<String>,
    },
    /// Get default header and copy it to clipboard
    Default {
        /// Placeholder values
        values: Vec<String>,
    },
    /// Manage configuration
    Config,
}

fn main() {
    env_logger::builder()
        .format_timestamp(None)
        .format_target(false)
        .filter_level(log::LevelFilter::Debug)
        .init();

    let input_theme = &ColorfulTheme::default();

    let config = Config::get();

    let mut templates = match fs::read_to_string(TEMPLATE_FILE_PATH) {
        Ok(result) => match serde_json::from_str::<Vec<Template>>(&result) {
            Ok(data) => data,
            Err(e) => {
                log::error!("Serde error - {}", e);
                return;
            }
        },
        Err(e) => {
            log::error!("Reading error - {}", e);
            return;
        }
    };

    let cli = Cli::parse();
    match cli.command {
        Commands::New => {
            let name: String = Input::with_theme(input_theme)
                .with_prompt("Header name")
                .validate_with(|input: &String| -> Result<(), &str> {
                    if let None = templates.iter().find(|template| template.name == *input) {
                        Ok(())
                    } else {
                        Err("Header name already exists")
                    }
                })
                .interact()
                .unwrap();

            let length: u16 = Input::with_theme(input_theme)
                .with_prompt("Header length")
                .default(config.header_len)
                .validate_with(|input: &u16| -> Result<(), String> {
                    if *input > 0 && *input <= config.header_len {
                        Ok(())
                    } else {
                        Err(format!(
                            "Header length must greater than 0 and less than or equal to {}",
                            config.header_len
                        ))
                    }
                })
                .interact()
                .unwrap();

            let space_before: u8 = Input::with_theme(input_theme)
                .with_prompt("Space before")
                .default(config.space_height)
                .validate_with(|input: &u8| -> Result<(), String> {
                    if *input <= config.space_height {
                        Ok(())
                    } else {
                        Err(format!(
                            "Space before paragraph must greater than or equal to 0 and less than or equal to {}",
                            config.space_height
                        ))
                    }
                })
                .interact()
                .unwrap();

            let space_after: u8 = Input::with_theme(input_theme)
                .with_prompt("Space before")
                .default(config.space_height)
                .validate_with(|input: &u8| -> Result<(), String> {
                    if *input <= config.space_height {
                        Ok(())
                    } else {
                        Err(format!(
                            "Space before paragraph must greater than or equal to 0 and less than or equal to {}",
                            config.space_height
                        ))
                    }
                })
                .interact()
                .unwrap();

            let mut border: Option<Border> = None;
            if Confirm::with_theme(input_theme)
                .with_prompt("Add border?")
                .interact()
                .unwrap()
            {
                let left = border_input("Left", input_theme);
                let top = border_input("Top", input_theme);
                let right = border_input("Right", input_theme);
                let bottom = border_input("Bottom", input_theme);
                if left.is_some() && top.is_some() && right.is_some() && left.is_some() {
                    border = Some(Border {
                        left,
                        top,
                        right,
                        bottom,
                    });
                }
            }

            let mut fields = vec![];
            while Confirm::with_theme(input_theme)
                .with_prompt("Add field")
                .interact()
                .unwrap()
            {
                let key: String = Input::with_theme(input_theme)
                    .with_prompt("Key")
                    .interact()
                    .unwrap();
                let value: String = Input::with_theme(input_theme)
                    .with_prompt("Value")
                    .interact()
                    .unwrap();
                let key_visible = Confirm::with_theme(input_theme)
                    .with_prompt("Key visible")
                    .interact()
                    .unwrap();
                let center = Confirm::with_theme(input_theme)
                    .with_prompt("Text center")
                    .interact()
                    .unwrap();
                fields.push(Field {
                    align: if center {
                        Alignment::Center
                    } else {
                        Alignment::Left
                    },
                    key: key,
                    default_value: value,
                    key_visible: key_visible,
                });
            }

            if border.is_none() && fields.len() == 0 {
                border = Some(Border {
                    left: None,
                    top: Some('#'),
                    right: None,
                    bottom: Some('#'),
                });
            }

            let template = Template {
                border: border,
                default: false,
                fields: fields,
                length: length,
                name: name,
                space_before: space_before,
                space_after: space_after,
            };

            templates.push(template);
            save(&templates);
        }
        Commands::List { selection } => {
            if !selection {
                for template in &templates {
                    println!("{}", template.name);
                }
                return;
            }
            let mut names = vec![];
            for template in &templates {
                names.push(template.name.clone());
            }

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select a header")
                .items(names)
                .default(0)
                .interact_opt()
                .unwrap();

            if let Some(idx) = selection {
                get_header(&templates[idx]);
            }
        }
        Commands::Get { name, json } => {
            match templates.iter().find(|template| template.name == name) {
                Some(template) => {
                    if json {
                        println!("{}", serde_json::to_string_pretty(template).unwrap());
                    } else {
                        get_header(template);
                    }
                }
                None => {
                    println!("{}", style(format!("`{}` not found", name)).red().bold());
                }
            }
        }
        Commands::Edit {
            header_name,
            name,
            spacing,
            border,
            fields,
        } => {
            if let Some(template) = templates
                .iter_mut()
                .find(|template| template.name == header_name)
            {
                // edit name
                if name.is_some() {
                    template.name = name.unwrap();
                }
                // edit spacing
                else if spacing {
                    let items = vec!["space_before", "space_after"];
                    let selection = Select::with_theme(input_theme)
                        .with_prompt("Select spacing")
                        .items(items)
                        .default(0)
                        .interact()
                        .unwrap();
                    let space = Input::with_theme(input_theme)
                        .with_prompt("New space")
                        .default(config.space_height)
                        .interact()
                        .unwrap();
                    match selection {
                        0 => {
                            template.space_before = space;
                        }
                        1 => template.space_after = space,
                        _ => {}
                    };
                }
                // edit border
                else if border {
                    let left = border_input("Left", input_theme);
                    let top = border_input("Top", input_theme);
                    let right = border_input("Right", input_theme);
                    let bottom = border_input("Bottom", input_theme);
                    if left.is_some() && top.is_some() && right.is_some() && left.is_some() {
                        template.border = Some(Border {
                            left,
                            top,
                            right,
                            bottom,
                        });
                    } else {
                        template.border = None;
                    }
                }
                // edit fields
                else if fields {
                    let fields = &mut template.fields;
                    if fields.len() <= 0 {
                        println!("This header hasn't got any fields");
                        return;
                    }
                    let keys = &fields
                        .iter()
                        .map(|field| field.key.clone())
                        .collect::<Vec<String>>();
                    loop {
                        let field_idx = Select::with_theme(input_theme)
                            .with_prompt("Select a field")
                            .items(keys)
                            .default(0)
                            .interact_opt()
                            .unwrap();
                        if field_idx.is_none() {
                            break;
                        }
                        let field_idx = field_idx.unwrap();
                        let field = &mut fields[field_idx];
                        loop {
                            let items = ["key", "default_value", "align", "key_visible"];
                            let selection = Select::with_theme(input_theme)
                                .with_prompt("Edit")
                                .items(items)
                                .default(0)
                                .interact_opt()
                                .unwrap();
                            if selection.is_none() {
                                break;
                            }
                            match selection.unwrap() {
                                0 => {
                                    let new_key = Input::with_theme(input_theme)
                                        .with_prompt("New key")
                                        .default(field.key.clone())
                                        .interact()
                                        .unwrap();
                                    field.key = new_key;
                                }
                                1 => {
                                    let new_default_value = Input::with_theme(input_theme)
                                        .with_prompt("New default value")
                                        .default(field.default_value.clone())
                                        .interact()
                                        .unwrap();
                                    field.default_value = new_default_value;
                                }
                                2 => {
                                    let is_center = Confirm::with_theme(input_theme)
                                        .with_prompt("Center text")
                                        .default(false)
                                        .interact()
                                        .unwrap();
                                    field.align = if is_center {
                                        Alignment::Center
                                    } else {
                                        Alignment::Left
                                    };
                                }
                                3 => {
                                    let key_visible = Confirm::with_theme(input_theme)
                                        .with_prompt("Key visible")
                                        .default(field.key_visible)
                                        .interact()
                                        .unwrap();
                                    field.key_visible = if key_visible { true } else { false };
                                }
                                _ => {
                                    break;
                                }
                            };
                        }
                    }
                }
                save(&templates);
            } else {
                println!(
                    "{}",
                    style(format!("`{}` not found", header_name)).red().bold()
                );
                return;
            }
        }
        Commands::Delete { all, name } => {
            if all {
                let confirm: bool = Confirm::with_theme(input_theme)
                    .with_prompt(format!("{}", style("Delete all headers").red().bold()))
                    .interact()
                    .unwrap();
                if confirm {
                    templates = vec![];
                }
            } else if let Some(name) = name {
                if let Some(idx) = templates.iter().position(|value| value.name == name) {
                    let confirm: bool = Confirm::with_theme(input_theme)
                        .with_prompt(format!(
                            "{} header `{}`?",
                            style("Delete").red().bold(),
                            name
                        ))
                        .interact()
                        .unwrap();

                    if confirm {
                        let deleted = templates.remove(idx);
                        if deleted.default && templates.len() > 0 {
                            templates[0].default = true;
                        }
                    } else {
                        return;
                    }
                } else {
                    println!("{}", style(format!("`{}` not found", name)).red().bold());
                    return;
                }
            } else {
                return;
            }
            save(&templates);
        }
        Commands::Default { values } => {
            if let Some(default_template) = templates.iter_mut().find(|template| template.default) {
                for i in 0..cmp::min(default_template.fields.len(), values.len()) {
                    default_template.fields[i].default_value = values[i].clone();
                }
                get_header(&default_template);
            }
        }
        Commands::Apply => {}
        Commands::Config => {}
    }
}

fn border_input(promt: &str, input_theme: &ColorfulTheme) -> Option<char> {
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

fn save(templates: &Vec<Template>) {
    if let Err(e) = fs::write(TEMPLATE_FILE_PATH, serde_json::json!(templates).to_string()) {
        log::error!("{}", e);
        return;
    }
    println!("{}", style("Saved").green().bold());
}

fn get_header(template: &Template) {
    let header = build_header(template);
    println!("\n{}\n", header);

    // copy to clipboard
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(header).unwrap();
}

fn build_header(template: &Template) -> String {
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

    // log::debug!("key_len: {}", key_len);
    // log::debug!("value_len: {}", value_len);
    // log::debug!("max_len: {}", max_len);

    // build header body
    let mut lines: Vec<String> = vec![];
    for field in &template.fields {
        let line = if field.key_visible {
            build_key_value_line(field, key_len, value_len)
        } else {
            build_line(field, max_len)
        };

        lines.push(line);

        // log::debug!("|{}|", line);
        // log::debug!("size: {}", line.len());
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

fn build_key_value_line(field: &Field, key_len: usize, value_len: usize) -> String {
    let mut result = String::new();
    let lines = wrap_text(&field.default_value, value_len.into());

    // log::debug!("Lines wrapped: {:?}", lines);

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

fn wrap_text(text: &str, line_len: usize) -> Vec<String> {
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
