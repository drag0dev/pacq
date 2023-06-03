use std::env::args;

pub mod batch;
pub mod config;
pub mod utils;

use utils::{load_config, load_batch};

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
