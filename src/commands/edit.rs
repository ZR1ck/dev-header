use console::style;
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};

use crate::{
    config::Config,
    core::{
        header::border_input,
        storage::Storage,
        template::{Alignment, Border},
    },
};

pub fn run(
    header_name: String,
    name: Option<String>,
    spacing: bool,
    border: bool,
    fields: bool,
    config: &Config,
    store: &mut Storage,
    input_theme: &ColorfulTheme,
) {
    if let Some(template) = store.get_mut(&header_name) {
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
            if left.is_some() || top.is_some() || right.is_some() || left.is_some() {
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
        if let Err(e) = store.save() {
            println!("{}: {}", style("error"), e);
        } else {
            println!("{}", style("Saved").green().bold());
        }
    } else {
        println!(
            "{}",
            style(format!("`{}` not found", header_name)).red().bold()
        );
        return;
    }
}
