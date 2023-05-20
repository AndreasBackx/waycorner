use std::{collections::HashMap, env, fs::File, io::Read, path::PathBuf};

use anyhow::{bail, Context, Result};
use regex::Regex;
use serde::{
    de::{self, Unexpected},
    Deserialize, Deserializer,
};
use tracing::{debug, info};

pub const COLOR_TRANSPARENT: u32 = 0x00_00_00_00;
pub const COLOR_RED: u32 = 0xFF_FF_00_00;

fn default_locations() -> Vec<Location> {
    vec![Location::BottomRight, Location::BottomLeft]
}

fn default_size() -> u8 {
    10
}

fn default_margin() -> i8 {
    20
}

fn default_timeout_ms() -> u16 {
    250
}

fn default_color() -> u32 {
    COLOR_RED
}

fn default_command() -> Vec<String> {
    Vec::new()
}

fn from_hex<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let value: &str = Deserialize::deserialize(deserializer)?;
    let re: Regex = Regex::new(r"^#[0-9a-fA-F]{6,8}$").unwrap();

    if !re.is_match(value) {
        return Err(de::Error::invalid_value(
            Unexpected::Str(value),
            &"a valid hex code",
        ));
    }
    let without_prefix = value.trim_start_matches('#');
    u32::from_str_radix(without_prefix, 16)
        .map(|val| {
            if val <= 0xFF_FF_FF {
                val | 0xFF_00_00_00
            } else {
                val
            }
        })
        .map_err(de::Error::custom)
}

#[derive(Clone, Debug, Deserialize)]
pub struct CornerConfig {
    pub output: Option<OutputConfig>,
    #[serde(default = "default_command", alias = "command")]
    pub enter_command: Vec<String>,
    #[serde(default = "default_command")]
    pub exit_command: Vec<String>,
    #[serde(default = "default_locations")]
    pub locations: Vec<Location>,
    #[serde(default = "default_size")]
    pub size: u8,
    #[serde(default = "default_margin")]
    pub margin: i8,
    #[serde(default = "default_timeout_ms")]
    pub timeout_ms: u16,
    #[serde(default = "default_color", deserialize_with = "from_hex")]
    pub color: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OutputConfig {
    pub description: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Location {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
    Left,
    Right,
    Top,
    Bottom,
}

type Config = HashMap<String, CornerConfig>;

pub fn get_configs(config_path: PathBuf) -> Result<Vec<CornerConfig>> {
    let path = if config_path.starts_with("~/") {
        debug!("Replacing ~/ with $HOME/");
        let home_path = env::var_os("HOME")
            .context("could not find the $HOME env var to use for the default config path")?;
        let relative_path = config_path
            .to_str()
            .unwrap_or_else(|| panic!("invalid config path specified: {}", config_path.display()))
            .to_string()
            .split_off(2);
        PathBuf::from(home_path).join(relative_path)
    } else {
        config_path
    };
    info!("Using config: {}", path.display());
    let mut config_file = File::open(path.clone())
        .with_context(|| format!("could not open the file {}", path.display()))?;
    let mut config_content = String::new();
    config_file.read_to_string(&mut config_content)?;
    toml::from_str::<Config>(config_content.as_str()).map(|item| {
        item.into_iter()
            .map(|(key, value)| {
                if value.enter_command.is_empty() && value.exit_command.is_empty() {
                    bail!(
                        "You must provide either an `exit_command` or an `enter_command` for `{}`",
                        key
                    )
                }
                Ok(value)
            })
            .collect::<Result<Vec<_>>>()
            .with_context(|| format!("could not parse {}", path.display()))
    })?
}
