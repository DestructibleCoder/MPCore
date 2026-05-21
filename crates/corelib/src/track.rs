use std::path::PathBuf;
use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Track {
    pub path: PathBuf,
    pub duration: Option<Duration>,
}
