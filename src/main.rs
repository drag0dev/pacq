use std::{
    env::args,
    fs::read_to_string
};
use anyhow::{Result, Context};
use serde_json::from_str;

pub mod batch;
pub mod config;

// TODO: all output to stdout also to the log file

use batch::Batch;
use config::Config;

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() != 3 {
        println!("Incorrect arguments");
        println!("Usage: pcq path-to-pacq-folder path-to-log-file");
        return;
    }

    let folder_path = &args[1];
    let log_path = &args[2];

    let config = load_config(folder_path);
    if config.is_err() {
        let err = config.err().unwrap();
        println!("error: {}\n\t{}", err, err.root_cause());
        return;
    }

    let config = config.unwrap();

    for batch_name in config.batches {
        println!("Parsing \"{}\" batch", batch_name);

        let loaded_batch = load_batch(&folder_path, &batch_name);
        if loaded_batch.is_err() {
            let err = loaded_batch.err().unwrap();
            println!("error: {}\n\t{}", err, err.root_cause());
            return;
        }
        let loaded_batch = loaded_batch.unwrap();

        println!("Finished \"{}\" batch", batch_name);
    }

}

fn load_config(folder_path: &String) -> Result<Config> {
    let config_full_path = format!("{}/config.json", folder_path);
    let config_contents = read_to_string(config_full_path)
        .context("reading config file")?;

    let config: Config = from_str(&config_contents)
        .context("parsing config file contents")?;

    Ok(config)
}

fn load_batch(folder_path: &String, batch_name: &String) -> Result<Batch> {
    let batch_full_path = format!("{}/{}.json", folder_path, batch_name);
    let batch_contents = read_to_string(batch_full_path)
        .context("reading batch file")?;

    let batch: Batch = from_str(&batch_contents)
        .context("parsing batch file contents")?;

    Ok(batch)
}
