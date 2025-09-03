use std::cmp;

use console::style;

use crate::core::{header::get_header, storage::Storage};

pub fn run(name: String, values: Vec<String>, store: &mut Storage) {
    if let Some(mut template) = store.get(&name).cloned() {
        for i in 0..cmp::min(template.fields.len(), values.len()) {
            template.fields[i].default_value = values[i].clone();
        }
        get_header(&template);
    } else {
        println!("{}", style("Header not found").red().bold());
    }
}
