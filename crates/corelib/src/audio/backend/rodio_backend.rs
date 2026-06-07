pub struct RodioBackend;

use crate::audio::AudioSource;
use anyhow::Result;
use rodio::Sink;

use crate::audio::AudioBackend;

impl AudioBackend for RodioBackend {
    fn load(&mut self, source: AudioSource) -> Result<()> {
        self.sink.stop();

        self.senk = Sink::connect_new(&self.mixer);

        self.sink.append(source);

        Ok(())
    }

    fn play(&self);

    fn pause(&self);

    fn stop(&self);

    fn seek(&self, position: Duration) -> Result<()>;

    fn set_volume(&self, volume: f32);

    fn is_paused(&self) -> bool;
}
