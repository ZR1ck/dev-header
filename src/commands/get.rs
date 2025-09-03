use console::style;

use crate::core::{header::get_header, storage::Storage};

pub fn run(name: String, json: bool, store: &Storage) {
    match store.get(&name) {
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
