mod symphonia;

pub use symphonia::SymphoniaDecoder;

use anyhow::Result;
use std::path::Path;

use crate::audio::AudioSource;

pub trait AudioDecoder: Send + Sync {
    fn decode(&self, path: &Path) -> Result<AudioSource>;
}
