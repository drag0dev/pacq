use serde_derive::{Serialize, Deserialize};

/// batch represent a single package manager and all items installed through it
#[derive(Debug, Serialize, Deserialize)]
pub struct Batch {
    /// command for the package manager
    /// e.g. 'pacman -S', 'emerge -va', 'npm i g'
    pub command: String,

    /// optinally string to put into stdin since some package managers expect a confirmation
    /// usally 'Y\n' or 'y\n'
    pub proceed_input: Option<String>,

    /// all items to be installed via the given package manager
    pub items: Vec<String>,

    /// should next batch, if there is one, be done if this one fails
    /// true - move on to the next one
    /// false - prompt user whether pacq should continue
    pub break_the_chain: bool,

    /// should all items be installed at once or one by one
    pub one_by_one: bool
}
