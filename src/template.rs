use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Border {
    pub left: Option<char>,
    pub top: Option<char>,
    pub right: Option<char>,
    pub bottom: Option<char>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    pub key: String,
    pub default_value: String,
    #[serde(default = "Field::default_align")]
    pub align: Alignment,
    #[serde(default)]
    pub key_visible: bool,
}

#[derive(Debug, Serialize, Deserialize)]
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
