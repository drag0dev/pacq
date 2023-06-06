use std::{
    fs::OpenOptions,
    io::Write
};
use anyhow::{Result, Context};

pub static CONFIG_TEMPLATE: &'static str = r#"
{
    "batches": []
}
"#;
pub static BATCH_TEMPLATE: &'static str = r#"
{
    "command": "",
    "args": [],
    ---"proceed_input": ""
    "items": [],
    "break_the_chain": true,
    "one_by_one": false
}
"#;

pub fn create_template(file_name: &str, template: &'static str) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)
        .context("opening file")?;

    file.write_all(template.as_bytes())
        .context("writing template to the file")?;

    Ok(())
}
