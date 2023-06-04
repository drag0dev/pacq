use std::{
    env::args, fs::OpenOptions
};

pub mod batch;
pub mod config;
pub mod utils;

use utils::{
    load_config, load_batch, execute_batch
};

// TODO: cli offers templates
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

    let log_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(log_path);

    if log_file.is_err() {
        let err = log_file.err().unwrap();
        println!("error: opening log file: {}", err);
        return;
    }
    let log_file = log_file.unwrap();

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

        println!("Running \"{}\" batch", batch_name);
        let res = execute_batch(&loaded_batch, &log_file);
        if res.is_err() {
            let err = res.err().unwrap();
            println!("error: {}\n\t{}", err, err.root_cause());
            if loaded_batch.break_the_chain {
                println!("Breaking the chain!");
                break;
            }
            println!("Finished \"{}\" batch", batch_name);
        }

        println!("Finished \"{}\" batch successfully", batch_name);
    }
}
