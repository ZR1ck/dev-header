use console::style;
use dialoguer::{Input, theme::ColorfulTheme};

use crate::core::{header::get_header, storage::Storage};

pub fn run(name: String, store: &mut Storage, input_theme: &ColorfulTheme) {
    if let Some(mut template) = store.get(&name).cloned() {
        for field in &mut template.fields {
            let input = Input::with_theme(input_theme)
                .with_prompt(field.key.clone())
                .interact()
                .unwrap();
            field.default_value = input;
        }
        get_header(&template);
    } else {
        println!("{}", style("Header not found").red().bold());
    }
}
