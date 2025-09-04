use clap::{ArgGroup, Parser, Subcommand, command};

pub mod apply;
pub mod default;
pub mod delete;
pub mod edit;
pub mod get;
pub mod list;
pub mod new;

#[derive(Parser)]
#[command(
    name = "header",
    version,
    about = "CLI tool for code comment headers",
    author = "<duydangtr@gmail.com>"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new header
    #[command(alias = "n")]
    New,
    /// List all headers
    #[command(alias = "l")]
    List {
        /// Selection mode: select a header with default values and copy it to clipboard
        #[arg(short, long)]
        selection: bool,
    },
    /// Get a header
    #[command(alias = "g")]
    Get {
        /// Header name
        name: String,
        /// JSON view
        #[arg(short, long)]
        json: bool,
    },
    /// Get a header and apply placeholders
    #[command(alias = "a")]
    Apply {
        /// Header name
        name: String,
    },
    #[command(group(ArgGroup::new("edit_options").required(true).args(&["border", "fields", "name", "spacing"])))]
    /// Edit an existing header
    #[command(alias = "e")]
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
    #[command(alias = "d")]
    Delete {
        // Delete all headers
        #[arg(short, long, conflicts_with = "name")]
        all: bool,
        /// Header name
        #[arg(required_unless_present = "all")]
        name: Option<String>,
    },
    /// Get default header and copy it to clipboard
    #[command(alias = "df")]
    Default {
        /// Placeholder values
        values: Vec<String>,
    },
    /// Manage configuration
    Config,
}
