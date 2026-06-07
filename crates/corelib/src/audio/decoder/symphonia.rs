use std::{fs::File, path::Path};

use anyhow::Result;
use rodio::Decoder;

use crate::audio::{AudioDecoder, AudioSource};

pub struct SymphoniaDecoder;

impl AudioDecoder for SymphoniaDecoder {
    fn decode(&self, path: &Path) -> Result<AudioSource> {
        let file = File::open(path)?;

        let source = Decoder::try_from(file)?;

        Ok(source)
    }
}
