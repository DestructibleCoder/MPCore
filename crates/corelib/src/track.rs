use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::Result;
use std::fs::File;

use lofty::file::TaggedFileExt;
use lofty::probe::Probe;
use lofty::tag::Accessor;
use rodio::{Decoder, Source};
use serde::{Deserialize, Serialize};

pub const SUPPORTED_EXTENSIONS: &[&str] = &["flac", "mp3", "ogg", "wav"];

#[derive(Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Track {
    pub path: PathBuf,
    #[serde(skip)]
    pub duration: Option<Duration>,
    pub metadata: Metadata,
}

impl Track {
    pub fn from_path(path: PathBuf) -> Result<Self> {
        if !path.is_file() {
            anyhow::bail!("It's not a file!");
        }

        let Some(extension) = path.extension() else {
            anyhow::bail!("Can't get file extension!");
        };

        let extension = extension.to_string_lossy().to_lowercase();

        if SUPPORTED_EXTENSIONS.contains(&extension.as_str()) {
            let duration = File::open(&path)
                .ok()
                .and_then(|file| Decoder::try_from(file).ok())
                .and_then(|decoder| decoder.total_duration());

            let metadata = read_metadata(&path);

            Ok(Self {
                path,
                duration,
                metadata,
            })
        } else {
            anyhow::bail!("The file has unsupported extension!");
        }
    }
}
