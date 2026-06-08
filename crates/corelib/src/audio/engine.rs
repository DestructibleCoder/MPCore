use anyhow::Result;

use crate::audio::{AudioBackend, AudioDecoder};
use crate::track::Track;

pub struct AudioEngine {
    decoder: Box<dyn AudioDecoder>,
    backend: Box<dyn AudioBackend>,
}

impl AudioEngine {
    pub fn new(decoder: Box<dyn AudioDecoder>, backend: Box<dyn AudioBackend>) -> Self {
        Self { decoder, backend }
    }

    pub fn play_track(&mut self, track: &Track) -> Result<()> {
        let source = self.decoder.decode(&track.path)?;

        self.backend.load(source)?;

        self.backend.play();

        Ok(())
    }

    pub fn pause(&self) {
        self.backend.pause();
    }

    pub fn play(&self) {
        self.backend.play();
    }

    pub fn stop(&self) {
        self.backend.stop();
    }

    pub fn seek(&self, position: std::time::Duration) -> Result<()> {
        self.backend.seek(position)?;

        Ok(())
    }

    pub fn set_volume(&self, vol: f32) {
        self.backend.set_volume(vol);
    }

    pub fn reset_sink(&self) {
        self.backend.reset_sink();
    }
}
