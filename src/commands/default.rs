use std::cmp;

use console::style;

use crate::core::{header::get_header, storage::Storage};

pub fn run(values: Vec<String>, store: &mut Storage) {
    if let Some(mut default_template) = store.get_default().cloned() {
        for i in 0..cmp::min(default_template.fields.len(), values.len()) {
            default_template.fields[i].default_value = values[i].clone();
        }
        get_header(&default_template);
    } else {
        println!("{}", style("Header not found").red().bold());
    }
}
