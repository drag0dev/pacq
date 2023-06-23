use std::{
    env::args,
    fs::OpenOptions,
    io::Write,
    path::Path
};

pub mod batch;
pub mod config;
pub mod utils;
pub mod template;

use template::{create_template, BATCH_TEMPLATE, CONFIG_TEMPLATE};
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

use batch::Batch;
use config::Config;

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() < 4 {
        println!("Incorrect arguments");
        println!("Usage: pacq run path-to-pacq-folder path-to-log-file");
        println!("Usage: pacq execute batch_file_name path-to-log-file");
        println!("Usage: pacq template batch/config file_name");
        return;
    }

    if args[1] != "run" && args[1] != "template" && args[1] != "execute" {
        println!("Incorrect arguments");
        println!("Usage: pacq run path-to-pacq-folder path-to-log-file");
        println!("Usage: pacq execute batch_file_name path-to-log-file");
        println!("Usage: pacq template batch/config file_name");
        return;
    }

    if args[1] == "template" && args.len() == 4 { handle_template(&args[2], &args[3]); }
    else if args[1] == "run" && args.len() == 4 { handle_run(&args[2], &args[3]) }
    else if args[1] == "execute" && args.len() == 4 { handle_execute(&args[2], &args[3]); }
    else {
        println!("Incorrect arguments");
        println!("Usage: pacq run path-to-pacq-folder path-to-log-file");
        println!("Usage: pacq execute batch_file_name path-to-log-file");
        println!("Usage: pacq template batch/config file_name");
    }
}

fn handle_run(folder_path: &str, log_path: &str) {
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
        let batch_full_path = format!("{}/{}.json", folder_path, batch_name);
        let loaded_batch = load_batch(&batch_full_path);
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

fn handle_template(template: &str, file_name: &str) {
    let res = if template == "batch" { create_template(file_name, BATCH_TEMPLATE) }
    else if template == "config" { create_template(file_name, CONFIG_TEMPLATE) }
    else {
        println!("error: unknown template \"{}\"", template);
        return;
    };

    if res.is_err() {
        let err = res.err().unwrap();
        println!("error: {}\n\t{}", err, err.root_cause());
    }
    return;
}

fn handle_execute(batch_path: &String, log_path: &String) {
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

    let batch_name = {
        let parsed_file = Path::new(batch_path).file_name();
        if parsed_file.is_none() {
            println!("error: invalid path");
            return;
        }
        let file_name = parsed_file.unwrap().to_str();
        if file_name.is_none() {
            println!("error: invalid path");
            return;
        }
        file_name.unwrap()
    };

    double_print!(format!("Parsing \"{}\" batch", batch_name), &log_file);
    let loaded_batch = load_batch(&batch_path);
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
            return;
        }
        double_print!(format!("Finished \"{}\" batch", batch_name), &log_file);
    }

    double_print!(format!("Finished \"{}\" batch successfully", batch_name), &log_file);
}
