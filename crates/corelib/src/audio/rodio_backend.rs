use std::fs::File;
use std::path::Path;
use std::time::Duration;

use anyhow::Result;

use rodio::mixer::Mixer;
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink, Source};

pub trait AudioBackend: Send {
    fn seek(&self, position: Duration) -> Result<()>;
    fn is_finished(&self) -> bool;
    fn reset_sink(&mut self);
    fn play_file(&mut self, path: &Path) -> Result<Option<Duration>>;
    fn pause(&self);
    fn play(&self);
    fn stop(&self);
    fn set_volume(&self, vol: f32);
    fn is_paused(&self) -> bool;
    fn is_empty(&self) -> bool;
}

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
    fn seek(&self, position: Duration) -> Result<()> {
        self.sink
            .try_seek(position)
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }

    fn is_finished(&self) -> bool {
        self.sink.empty()
    }

    fn reset_sink(&mut self) {
        self.sink.stop();

        self.sink = Sink::connect_new(&self.mixer);
    }

    fn play_file(&mut self, path: &Path) -> Result<Option<Duration>> {
        let file = File::open(path)?;

        let source = Decoder::try_from(file)?;

        let duration = source.total_duration();

        self.sink.append(source);

        Ok(duration)
    }

    fn pause(&self) {
        self.sink.pause();
    }

    fn play(&self) {
        self.sink.play();
    }

    fn stop(&self) {
        self.sink.stop();
    }

    fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume);
    }

    fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }

    fn is_empty(&self) -> bool {
        self.sink.empty()
    }
}
