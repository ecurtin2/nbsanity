use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    pub root: Option<String>,
    pub disable: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Serialize)]
struct ToolConfig {
    nbsanity: Config,
}

#[derive(Deserialize, Debug, Serialize)]
struct PyProjectTomlConfig {
    tool: ToolConfig,
}

impl Config {
    pub fn build() -> Config {
        let config_toml = fs::read_to_string("./pyproject.toml");
        let config = match config_toml {
            Ok(toml) => {
                let pyproj_config: PyProjectTomlConfig = toml::from_str(&toml).unwrap();
                pyproj_config.tool.nbsanity
            }
            Err(_e) => Config {
                root: None,
                disable: Some(Vec::new()),
            },
        };
        return config;
    }

    pub fn root_path(&self) -> &Path {
        match &self.root {
            Some(p) => Path::new(p),
            None => Path::new("."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checks::{Check, FileNotNamedUntitled};

    // test config from toml string
    #[test]
    fn test_config_from_toml() {
        let toml = r#"
        [tool.nbsanity]
        root = "tests"
        disable = ["FileNotNamedUntitled"]
        "#;
        let config: PyProjectTomlConfig = toml::from_str(toml).unwrap();
        assert_eq!(config.tool.nbsanity.root.unwrap(), "tests");

        let disable = Check::from_str(config.tool.nbsanity.disable.unwrap()[0].as_str()).unwrap();
        assert_eq!(
            disable,
            Check::FileNotNamedUntitled(FileNotNamedUntitled {})
        );
    }
}
