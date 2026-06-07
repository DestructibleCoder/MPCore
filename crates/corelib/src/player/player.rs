use anyhow::Result;

use std::path::Path;
use std::time::Duration;

use crate::audio::{AudioEngine, RodioBackend, SymphoniaDecoder};
use crate::playlist::Playlist;
use crate::queue::Queue;

#[derive(Debug, Clone, Copy)]
pub enum PlaybackState {
    Playing,
    Paused,
    Stopped,
}

#[derive(Clone, Copy)]
pub enum RepeatMode {
    None,
    Queue,
    Track,
}

pub struct Player {
    queue: Queue,
    engine: AudioEngine,
    state: PlaybackState,

    repeat_mode: RepeatMode,
}

impl Player {
    pub fn new(queue: Queue) -> Result<Self> {
        let backend = RodioBackend::new()?;
        let decoder = SymphoniaDecoder;

        let engine = AudioEngine::new(Box::new(decoder), Box::new(backend));

        Ok(Self {
            queue,
            engine: engine,
            state: PlaybackState::Stopped,

            repeat_mode: RepeatMode::None,
        })
    }

    pub fn seek(&mut self, seconds: u64) -> Result<()> {
        self.engine.seek(Duration::from_secs(seconds))?;

        Ok(())
    }

    pub fn save_playlist(&self, path: &Path) -> Result<()> {
        self.queue.save_playlist(path)
    }

    pub fn shuffle_around(&mut self) {
        self.queue.shuffle_around();
    }

    pub fn load_playlist(&mut self, path: &Path) -> Result<()> {
        self.queue = Queue::load_playlist(path)?;

        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        if matches!(self.state(), PlaybackState::Playing) && self.backend.is_finished() {
            match self.repeat_mode() {
                RepeatMode::None => {
                    let curr = self
                        .queue
                        .current()
                        .ok_or(anyhow::anyhow!("Queue is empty!"))?;
                    if curr + 1 < self.queue.tracks().len() {
                        self.next_track()?;
                    } else {
                        self.state = PlaybackState::Stopped;
                    }
                }

                RepeatMode::Queue => {
                    self.next_track()?;
                }

                RepeatMode::Track => {
                    self.play_current()?;
                }
            }
        }

        Ok(())
    }

    pub fn shuffle(&mut self) {
        self.queue.shuffle();
    }

    pub fn unshuffle(&mut self) {
        self.queue.unshuffle();
    }

    pub fn set_repeat_mode(&mut self, mode: RepeatMode) {
        self.repeat_mode = mode;
    }

    pub fn repeat_mode(&self) -> RepeatMode {
        self.repeat_mode
    }

    pub fn play_current(&mut self) -> Result<()> {
        if self.queue.tracks().is_empty() {
            return Ok(());
        }

        if matches!(self.state(), PlaybackState::Paused) {
            self.resume();

            return Ok(());
        }

        let curr = self
            .queue
            .current()
            .ok_or(anyhow::anyhow!("Queue is empty!"))?;
        let track = &self.queue.tracks()[curr];

        self.backend.play_file(&track.path)?;

        self.state = PlaybackState::Playing;

        Ok(())
    }

    pub fn play_track(&mut self, index: usize) -> Result<()> {
        self.backend.reset_sink();

        if matches!(self.state(), PlaybackState::Paused) {
            self.resume();
        }

        self.queue.set_current(index);

        self.play_current()
    }

    pub fn current_track_name(&self) -> Option<String> {
        let curr = self.queue.current()?;
        let track = self.queue.get_track(curr)?;

        if let Some(title) = &track.metadata.title {
            if let Some(artist) = &track.metadata.artist {
                return Some(format!("{} - {}", artist, title));
            }

            return Some(title.clone());
        }

        track
            .path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
    }

    pub fn get_track_info(&self) -> Option<String> {
        let curr = self.queue.current()?;
        let track = self.queue.get_track(curr)?;

        if let Some(title) = &track.metadata.title {
            if let Some(artist) = &track.metadata.artist {
                if let Some(album) = &track.metadata.album {
                    return Some(format!("{} - {} | {}", artist, title, album));
                }

                return Some(format!("{} - {} | Unknown Album", artist, title));
            }

            return Some(title.clone());
        }

        return track
            .path
            .file_name()
            .map(|n| n.to_string_lossy().to_string());
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

    pub fn extend_playlist(&self, path: &Path) -> Result<()> {
        match Playlist::load(&path) {
            Ok(mut playlist) => {
                playlist.add_tracks_from_queue(self.queue());
                playlist.save(path)?;
                Ok(())
            }

            Err(_) => {
                anyhow::bail!("Can't open playlist {:?}!", path.to_str());
            }
        }
    }
}
