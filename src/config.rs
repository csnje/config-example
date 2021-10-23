use std::error::Error;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct SubConfig {
    subval1: f64,
    subval2: Vec<bool>,
}

impl Default for SubConfig {
    fn default() -> Self {
        Self {
            subval1: std::f64::consts::PI,
            subval2: vec![true, false],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    val1: u32,
    val2: String,
    val3: SubConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            val1: 42,
            val2: "foobar".to_string(),
            val3: SubConfig::default(),
        }
    }
}

impl Config {
    pub fn from_str(s: &str) -> Result<Config, Box<dyn Error>> {
        Ok(serde_yaml::from_str(&s)?)
    }

    pub fn read_from_path(path: &Path) -> Result<Config, Box<dyn Error>> {
        Ok(Config::from_str(&fs::read_to_string(path)?)?)
    }

    pub fn to_string(&self) -> Result<String, Box<dyn Error>> {
        Ok(serde_yaml::to_string(&self)?)
    }

    pub fn write_to_path(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        fs::write(path, self.to_string()?.as_bytes())?;
        Ok(())
    }
}
