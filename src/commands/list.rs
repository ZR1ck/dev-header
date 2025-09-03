use dialoguer::{Select, theme::ColorfulTheme};

use crate::core::{header::get_header, storage::Storage};

pub fn run(selection: bool, store: &Storage, input_theme: &ColorfulTheme) {
    if !selection {
        for template in store.list() {
            println!("{}", template.name);
        }
        return;
    }
    let mut names = vec![];
    for template in store.list() {
        names.push(template.name.clone());
    }

    let selection = Select::with_theme(input_theme)
        .with_prompt("Select a header")
        .items(names)
        .default(0)
        .interact_opt()
        .unwrap();

    if let Some(idx) = selection {
        get_header(&store.list()[idx]);
    }
}
