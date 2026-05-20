use anyhow::Result;

use crate::audio::RodioBackend;
use crate::queue::Queue;

#[derive(Debug, Clone, Copy)]
pub enum PlaybackState {
    Playing,
    Paused,
    Stopped,
}

pub struct Player {
    queue: Queue,
    backend: RodioBackend,
    state: PlaybackState,
}

impl Player {
    pub fn new(queue: Queue) -> Result<Self> {
        let backend = RodioBackend::new()?;

        Ok(Self {
            queue,
            backend,
            state: PlaybackState::Stopped,
        })
    }

    pub fn play_current(&mut self) -> Result<()> {
        if self.queue.tracks.is_empty() {
            return Ok(());
        }

        if matches!(self.state(), PlaybackState::Paused) {
            self.resume();

            return Ok(());
        }

        let track = &self.queue.tracks[self.queue.current];

        self.backend.play_file(&track.path)?;

        self.state = PlaybackState::Playing;

        Ok(())
    }

    pub fn play_track(&mut self, index: usize) -> Result<()> {
        self.backend.reset_sink();

        self.queue.set_current(index);

        self.play_current()
    }

    pub fn current_track_name(&self) -> Option<String> {
        self.queue
            .tracks
            .get(self.queue.current)
            .and_then(|t| t.path.file_name())
            .map(|n| n.to_string_lossy().to_string())
    }

    pub fn next_track(&mut self) -> Result<()> {
        self.backend.reset_sink();

        self.queue.next_track();

        self.play_current()
    }

    pub fn previous_track(&mut self) -> Result<()> {
        self.backend.reset_sink();

        self.queue.previous_track();

        self.play_current()
    }

    pub fn pause(&mut self) {
        if matches!(self.state, PlaybackState::Playing) {
            self.backend.pause();
            self.state = PlaybackState::Paused;
        }
    }

    pub fn resume(&mut self) {
        self.backend.play();
        self.state = PlaybackState::Playing;
    }

    pub fn stop(&mut self) {
        self.backend.reset_sink();
        self.state = PlaybackState::Stopped;
    }

    pub fn set_volume(&self, volume: f32) {
        self.backend.set_volume(volume);
    }

    pub fn state(&self) -> PlaybackState {
        self.state
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }
}
