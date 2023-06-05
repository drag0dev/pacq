use std::{
    env::args, fs::OpenOptions, io::Write
};

pub mod batch;
pub mod config;
pub mod utils;

use utils::{
    load_config, load_batch, execute_batch
};

macro_rules! double_print {
    ($msg: expr, $log_file: expr) => {
        println!("{}", $msg);
        if let Err(e) = $log_file.write_all(format!("---pacq--- {}\n", $msg).as_bytes()) {
            println!("error: writing to log file: {}", e);
            println!("Quitting!");
            return;
        }
    };
}

// TODO: cli offers templates

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
        double_print!(format!("error: {}\n\t{}", err, err.root_cause()), &log_file);
        return;
    }

    let config = config.unwrap();

    for batch_name in config.batches {
        double_print!(format!("Parsing \"{}\" batch", batch_name), &log_file);
        let loaded_batch = load_batch(&folder_path, &batch_name);
        if loaded_batch.is_err() {
            let err = loaded_batch.err().unwrap();
            double_print!(format!("error: {}\n\t{}", err, err.root_cause()), &log_file);
            return;
        }
        let loaded_batch = loaded_batch.unwrap();

        double_print!(format!("Running \"{}\" batch", batch_name), &log_file);
        let res = execute_batch(&loaded_batch, &log_file);
        if res.is_err() {
            let err = res.err().unwrap();
            double_print!(format!("error: {}\n\t{}", err, err.root_cause()), &log_file);
            if loaded_batch.break_the_chain {
                double_print!("Breaking the chain!", &log_file);
                break;
            }
            double_print!(format!("Finished \"{}\" batch", batch_name), &log_file);
        }

        double_print!(format!("Finished \"{}\" batch successfully", batch_name), &log_file);
    }
}
