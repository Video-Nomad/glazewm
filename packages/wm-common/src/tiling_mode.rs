use std::str::FromStr;

use anyhow::bail;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TilingMode {
  #[default]
  Manual,
  Dwindle,
}

impl FromStr for TilingMode {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "manual" => Ok(Self::Manual),
      "dwindle" => Ok(Self::Dwindle),
      _ => bail!("Invalid tiling mode: {}", s),
    }
  }
}
