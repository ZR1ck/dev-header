use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Template {
    pub name: String,
    pub default: bool,
    pub length: u16,
    pub border: Option<Border>,
    pub fields: Vec<Field>,
    #[serde(default)]
    pub space_before: u8,
    #[serde(default)]
    pub space_after: u8,
}

impl Template {
    pub const DEFAULT_HEADER_LEN: u16 = 75;
    pub const MAX_HEADER_LEN: u16 = 255;
    pub const SPACE_HEIGHT: u8 = 0;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Border {
    pub left: Option<char>,
    pub top: Option<char>,
    pub right: Option<char>,
    pub bottom: Option<char>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Field {
    pub key: String,
    pub default_value: String,
    #[serde(default = "Field::default_align")]
    pub align: Alignment,
    #[serde(default)]
    pub key_visible: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Alignment {
    Left,
    Center,
}

impl Field {
    fn default_align() -> Alignment {
        Alignment::Left
    }
}
