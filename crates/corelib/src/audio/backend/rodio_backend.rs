use crate::audio::{AudioBackend, AudioSource};
use anyhow::Result;
use rodio::{OutputStream, OutputStreamBuilder, Sink, mixer::Mixer};
use std::time::Duration;

pub struct RodioBackend {
    _stream: OutputStream,
    mixer: Mixer,
    sink: Sink,
}

impl RodioBackend {
    pub fn new() -> Result<Self> {
        let stream = OutputStreamBuilder::open_default_stream()?;

        let mixer = stream.mixer().clone();

        let sink = Sink::connect_new(stream.mixer());

        Ok(Self {
            _stream: stream,
            mixer,
            sink,
        })
    }
}

impl AudioBackend for RodioBackend {
    fn load(&mut self, source: AudioSource) -> Result<()> {
        self.sink.stop();

        self.sink = Sink::connect_new(&self.mixer);

        self.sink.append(source);

        Ok(())
    }

    fn play(&self) {
        self.sink.play();
    }

    fn pause(&self) {
        self.sink.pause();
    }

    fn stop(&self) {
        self.sink.stop();
    }

    fn seek(&self, position: Duration) -> Result<()> {
        self.sink
            .try_seek(position)
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }

    fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume);
    }

    fn volume(&self) -> f32 {
        self.sink.volume()
    }

    fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }

    fn reset_sink(&self) {
        self.sink.clear();
    }
}
