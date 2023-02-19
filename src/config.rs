use serde::{Deserialize, Serialize};
use std::default::Default;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct Config {
    pub root: Option<String>,
    pub disable: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Serialize, Default)]
struct ToolConfig {
    nbsanity: Config,
}

#[derive(Deserialize, Debug, Serialize, Default)]
struct PyProjectTomlConfig {
    tool: ToolConfig,
}

impl Config {
    pub fn build() -> Config {
        fs::read_to_string("./pyproject.toml").map_or(Config::default(), |s| {
            toml::from_str::<PyProjectTomlConfig>(&s)
                .unwrap_or_default()
                .tool
                .nbsanity
        })
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
