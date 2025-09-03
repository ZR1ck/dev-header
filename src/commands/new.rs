use console::style;
use dialoguer::{Confirm, Input, theme::ColorfulTheme};

use crate::{
    config::Config,
    core::{
        header::border_input,
        storage::Storage,
        template::{Alignment, Border, Field, Template},
    },
};

pub fn run(config: &Config, store: &mut Storage, input_theme: &ColorfulTheme) {
    let name: String = Input::with_theme(input_theme)
        .with_prompt("Header name")
        .validate_with(|input: &String| -> Result<(), &str> {
            if let None = store.get(&input) {
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
        if left.is_some() || top.is_some() || right.is_some() || left.is_some() {
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

    if let Err(e) = store.add(template) {
        println!("{}: {}", style("error").red().bold(), e)
    }
}
