use std::fs::File;
use std::path::Path;

use anyhow::Result;

use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};

pub struct RodioBackend {
    _stream: OutputStream,
    sink: Sink,
}

impl RodioBackend {
    pub fn new() -> Result<Self> {
        let stream = OutputStreamBuilder::open_default_stream()?;

        let sink = Sink::connect_new(stream.mixer());

        Ok(Self {
            _stream: stream,
            sink,
        })
    }

    pub fn play_file(&self, path: &Path) -> Result<()> {
        let file = File::open(path)?;

        let source = Decoder::new(file)?;

        self.sink.append(source);

        Ok(())
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
