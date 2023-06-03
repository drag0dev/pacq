use serde_derive::{Serialize, Deserialize};

/// config for running batches
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// name of the batches to be done in the order they are written
    /// name of the batch is the filename without the trailing .json
    pub batches: Vec<String>
}
