use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Track {
    pub path: PathBuf,
    pub duration: Option<Duration>,
}
