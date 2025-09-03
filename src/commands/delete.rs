use console::style;
use dialoguer::{Confirm, theme::ColorfulTheme};

use crate::core::storage::Storage;

pub fn run(all: bool, name: Option<String>, store: &mut Storage, input_theme: &ColorfulTheme) {
    if all {
        let confirm: bool = Confirm::with_theme(input_theme)
            .with_prompt(format!("{}", style("Delete all headers").red().bold()))
            .interact()
            .unwrap();
        if confirm {
            if let Err(e) = store.delete_all() {
                println!("{}: {}", style("error").red().bold(), e);
                return;
            }
        }
    } else if let Some(name) = name {
        let confirm: bool = Confirm::with_theme(input_theme)
            .with_prompt(format!(
                "{} header `{}`?",
                style("Delete").red().bold(),
                name
            ))
            .interact()
            .unwrap();

        if confirm {
            if let Err(e) = store.delete(&name) {
                println!("{}: {}", style("error").red().bold(), e);
                return;
            }
        } else {
            return;
        }
    } else {
        return;
    }
}
