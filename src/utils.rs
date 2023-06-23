use std::{
    fs::{read_to_string, File},
    process::Command,
    io::Write
};
use serde_json::from_str;
use anyhow::{Result, Context, anyhow};

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

pub fn execute_batch(batch: &Batch, log_file: &File) -> Result<()> {
    if batch.one_by_one {
        for i in 0..batch.items.len() {
            let res = execute_command(batch, &log_file, &batch.items[i..i])?;
            if !res && batch.break_the_chain {
                return Err(anyhow!("process exitted with non zero exit code"));
            }
        }
    } else {
        let res = execute_command(batch, &log_file, &batch.items[..])?;
        if !res {
            return Err(anyhow!("process exitted with non zero exit code"));
        }
    }

    Ok(())
}

fn execute_command(batch: &Batch, log_file: &File, items: &[String])
    -> Result<bool>
{
    let mut command = Command::new(&batch.command);
    command.stdout(log_file
            .try_clone()
            .context("cloning file handler for stdout")?);
    command.stderr(log_file
            .try_clone()
            .context("cloning file handler for stderr")?);

    for arg in batch.args.iter() {
        command.arg(arg);
    }

    for item in items {
        command.arg(item);
    }

    let mut command = command
        .spawn()
        .context("spawning a command")?;

    let mut process_stdin;
    if let Some(input) = &batch.forward_input {
        if let Some(stdin) = command.stdin.take() {
            process_stdin = stdin;
        } else {
            return Err(anyhow!("stdin missing from the process"));
        }
        process_stdin.write_all(input.as_bytes())
            .context("writing input into the process stdin")?;
    }

    let process_res = command.wait()
        .context("waiting for process to finish")?;

    Ok(process_res.success())
}
