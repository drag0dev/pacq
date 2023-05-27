use serde_derive::{Serialize, Deserialize};

/// batch represent a single package manager and all items installed through it
#[derive(Debug, Serialize, Deserialize)]
pub struct Batch {
    /// command for the package manager
    /// e.g. 'pacman -S', 'emerge -va', 'npm i g'
    command: String,

    /// all items to be installed via the given package manager
    items: Vec<String>,

    /// should next batch, if there is one, be done if this one fails
    /// true - move on to the next one
    /// false - prompt user whether pacq should continue
    break_the_chain: bool,

    /// should all items be installed at once or one by one
    one_by_one: bool
}
