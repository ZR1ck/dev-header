use crate::{
    commands::{Cli, Commands},
    core::storage::Storage,
};
use clap::Parser;
use dialoguer::theme::ColorfulTheme;

mod commands;
mod core;

pub fn run() {
    let cli = Cli::parse();
    let mut store = Storage::new();
    let input_theme = ColorfulTheme::default();

    match cli.command {
        Commands::New => {
            commands::new::run(&mut store, &input_theme);
        }
        Commands::List { selection } => {
            commands::list::run(selection, &store, &input_theme);
        }
        Commands::Get { name, json } => {
            commands::get::run(name, json, &store);
        }
        Commands::Edit {
            header_name,
            name,
            spacing,
            border,
            fields,
        } => {
            commands::edit::run(
                header_name,
                name,
                spacing,
                border,
                fields,
                &mut store,
                &input_theme,
            );
        }
        Commands::Delete { all, name } => {
            commands::delete::run(all, name, &mut store, &input_theme);
        }
        Commands::Default { values } => {
            commands::default::run(values, &mut store);
        }
        Commands::Apply { name, values } => {
            commands::apply::run(name, values, &mut store);
        }
        Commands::Config => {}
    }
}
