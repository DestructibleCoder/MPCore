use std::fs::File;
use std::path::Path;
use std::time::Duration;

use anyhow::Result;

use rodio::mixer::Mixer;
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink, Source};

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

    pub fn is_finished(&self) -> bool {
        self.sink.empty()
    }

    pub fn reset_sink(&mut self) {
        self.sink.stop();

        self.sink = Sink::connect_new(&self.mixer);
    }

    pub fn play_file(&mut self, path: &Path) -> Result<Option<Duration>> {
        let file = File::open(path)?;

        let source = Decoder::try_from(file)?;

        let duration = source.total_duration();

        self.sink.append(source);

        Ok(duration)
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn play(&self) {
        self.sink.play();
    }

    pub fn stop(&self) {
        self.sink.stop();
    }

    pub fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume);
    }

    pub fn is_paused(&self) {
        self.sink.is_paused();
    }

    pub fn is_empty(&self) -> bool {
        self.sink.empty()
    }
}
