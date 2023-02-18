use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
use toml::Value;
use std::path::Path;
use crate::checks::Check;


#[derive(Deserialize, Debug)]
pub struct Config {
    pub root: Option<String>,
    pub disable: Option<Vec<Check>>,
}

#[derive(Deserialize, Debug)]
struct ToolConfig {
    nbsanity: Config,
}

#[derive(Deserialize, Debug)]
struct PyProjectTomlConfig {
    tool: ToolConfig,
}

impl Config {
    pub fn build () -> Config {
        let config_toml = fs::read_to_string("./pyproject.toml").expect("Error reading config");
        let pyproj_config: PyProjectTomlConfig = toml::from_str(&config_toml).unwrap();
        let config = pyproj_config.tool.nbsanity;
        return config
    }

    pub fn root_path(&self) -> &Path {
        match &self.root {
            Some(p) => Path::new(p),
            None => Path::new(".")

        }
    }
}

