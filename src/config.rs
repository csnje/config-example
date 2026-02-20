use std::fs;
use std::path::Path;

use config::Config;
use serde::{Deserialize, Serialize};

const ENVIRONMENT_VARIABLE_PREFIX: &str = "APP";
const ENVIRONMENT_VARIABLE_SEPARATOR: &str = "__";

#[derive(Debug, Deserialize, Serialize)]
pub struct Configuration {
    val1: u32,
    val2: String,
    val3: ChildConfiguration,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            val1: 42,
            val2: "foobar".to_string(),
            val3: ChildConfiguration::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct ChildConfiguration {
    val1: f64,
    val2: Vec<bool>,
}

impl Default for ChildConfiguration {
    fn default() -> Self {
        Self {
            val1: std::f64::consts::PI,
            val2: vec![true, false],
        }
    }
}

impl Configuration {
    pub fn build(path: &Path) -> anyhow::Result<Self> {
        Ok(Config::builder()
            .add_source(config::File::from(path))
            .add_source(
                config::Environment::with_prefix(ENVIRONMENT_VARIABLE_PREFIX)
                    .separator(ENVIRONMENT_VARIABLE_SEPARATOR),
            )
            .build()?
            .try_deserialize()?)
    }

    #[allow(dead_code)]
    pub fn read_from_path(path: &Path) -> anyhow::Result<Self> {
        Self::from_str(&fs::read_to_string(path)?)
    }

    #[allow(dead_code)]
    pub fn from_str(s: &str) -> anyhow::Result<Self> {
        Ok(serde_yml::from_str(s)?)
    }

    pub fn write_to_path(&self, path: &Path) -> anyhow::Result<()> {
        fs::write(path, self.to_string()?.as_bytes())?;
        Ok(())
    }

    pub fn to_string(&self) -> anyhow::Result<String> {
        Ok(serde_yml::to_string(&self)?)
    }
}
