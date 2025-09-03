use std::{fs, path::PathBuf};

use directories::ProjectDirs;

use crate::core::template::Template;

const DEFAULT_DATA: &str = include_str!("../data/data.json");

pub struct Storage {
    templates: Vec<Template>,
    dir: PathBuf,
}

impl Storage {
    pub fn new() -> Self {
        if let Some(prj_dir) = ProjectDirs::from("com", "", "dev-header") {
            let data_dir = prj_dir.data_dir().to_path_buf();
            fs::create_dir_all(&data_dir).unwrap();

            let data_file = data_dir.join("data.json");

            if !data_file.exists() {
                fs::write(&data_file, DEFAULT_DATA).unwrap();
            }
            let data_string = fs::read_to_string(&data_file).unwrap();
            let templates = serde_json::from_str::<Vec<Template>>(&data_string).unwrap();
            Self {
                templates: templates,
                dir: data_file,
            }
        } else {
            panic!("Could not determine user directory");
        }
    }

    pub fn add(&mut self, template: Template) -> Result<(), Box<dyn std::error::Error>> {
        self.templates.push(template);
        self.save()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(&self.dir, serde_json::json!(self.templates).to_string())?;
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&Template> {
        self.templates.iter().find(|template| template.name == name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Template> {
        self.templates
            .iter_mut()
            .find(|template| template.name == name)
    }

    pub fn get_default(&self) -> Option<&Template> {
        self.templates
            .iter()
            .find(|template| template.default == true)
    }

    pub fn list(&self) -> &Vec<Template> {
        &self.templates
    }

    pub fn delete(&mut self, name: &str) -> Result<(), String> {
        if let Some(idx) = self
            .templates
            .iter()
            .position(|template| template.name == name)
        {
            let deleted = self.templates.remove(idx);
            if deleted.default && self.templates.len() > 0 {
                self.templates[0].default = true;
            }
            if let Err(e) = self.save() {
                Err(e.to_string())
            } else {
                Ok(())
            }
        } else {
            Err("not found".to_string())
        }
    }

    pub fn delete_all(&mut self) -> Result<(), String> {
        self.templates = vec![];
        if let Err(e) = self.save() {
            Err(e.to_string())
        } else {
            Ok(())
        }
    }
}
