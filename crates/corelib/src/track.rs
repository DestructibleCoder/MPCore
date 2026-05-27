use std::path::{Path, PathBuf};
use std::time::Duration;

use lofty::file::TaggedFileExt;
use lofty::probe::Probe;
use lofty::tag::Accessor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Track {
    pub path: PathBuf,
    pub duration: Option<Duration>,
    pub metadata: Metadata,
}

pub fn read_metadata(path: &Path) -> Metadata {
    let tagged_file = Probe::open(path).ok().and_then(|file| file.read().ok());

    let Some(tagged_file) = tagged_file else {
        return Metadata {
            title: None,
            artist: None,
            album: None,
        };
    };

    let Some(tag) = tagged_file.primary_tag() else {
        return Metadata {
            title: None,
            album: None,
            artist: None,
        };
    };

    Metadata {
        title: tag.title().map(|s| s.to_string()),
        artist: tag.artist().map(|s| s.to_string()),
        album: tag.album().map(|s| s.to_string()),
    }
}
