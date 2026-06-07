mod rodio_backend;

pub use rodio_backend::RodioBackend;

use anyhow::Result;
use std::time::Duration;

use crate::audio::AudioSource;

pub trait AudioBackend: Send {
    fn load(&mut self, source: AudioSource) -> Result<()>;

    fn play(&self);

    fn pause(&self);

    fn stop(&self);

    fn seek(&self, position: Duration) -> Result<()>;

    fn set_volume(&self, volume: f32);

    fn is_paused(&self) -> bool;
}
