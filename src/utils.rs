use std::fs::read_to_string;
use serde_json::from_str;
use anyhow::{Result, Context};

use crate::{Config, Batch};

pub fn load_config(folder_path: &String) -> Result<Config> {
    let config_full_path = format!("{}/config.json", folder_path);
    let config_contents = read_to_string(config_full_path)
        .context("reading config file")?;

    let config: Config = from_str(&config_contents)
        .context("parsing config file contents")?;

    Ok(config)
}

pub fn load_batch(folder_path: &String, batch_name: &String) -> Result<Batch> {
    let batch_full_path = format!("{}/{}.json", folder_path, batch_name);
    let batch_contents = read_to_string(batch_full_path)
        .context("reading batch file")?;

    let batch: Batch = from_str(&batch_contents)
        .context("parsing batch file contents")?;

    Ok(batch)
}
