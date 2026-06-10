use anyhow::Result;

use std::path::Path;
use std::time::Duration;

use crate::audio::{AudioEngine, RodioBackend, SymphoniaDecoder};
use crate::player::{PlaybackState, RepeatMode};
use crate::playlist::Playlist;
use crate::queue::Queue;
use crate::track::Track;

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

    pub fn volume(&self) -> f32 {
        self.engine.volume()
    }

    pub fn seek(&mut self, seconds: u64) -> Result<()> {
        self.engine.seek(Duration::from_secs(seconds))?;

        Ok(())
    }

    pub fn save_playlist(&self, path: &Path) -> Result<()> {
        self.queue.save_playlist(path)
    }

    pub fn load_playlist(&mut self, path: &Path) -> Result<()> {
        self.queue = Queue::load_playlist(path)?;

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

        let vol = self.engine.volume();

        let curr = self
            .queue
            .current()
            .ok_or(anyhow::anyhow!("Queue is empty!"))?;
        let track = &self.queue.tracks()[curr];

        self.engine.play_track(&track)?;
        self.engine.set_volume(vol);

        self.state = PlaybackState::Playing;

        Ok(())
    }

    pub fn get_tracks_in_order(&self) -> Vec<Track> {
        let tracks = self.queue.tracks();
        let order = self.queue.order();

        let mut ret: Vec<Track> = Vec::new();

        for &idx in order.into_iter() {
            ret.push(tracks[idx].clone());
        }

        ret
    }

    pub fn play_by_index(&mut self, idx: Option<usize>) -> Result<()> {
        anyhow::ensure!(
            idx.is_some() && idx.unwrap() < self.queue.len(),
            "Incorrect index!"
        );

        let real_idx = self.queue.order().get(idx.unwrap()).unwrap();

        self.queue.set_current(*real_idx);
        self.play_current()
    }

    pub fn current_track_name(&self) -> Option<String> {
        let track = self.queue.current_track()?;

        if let Some(title) = &track.metadata.title {
            if let Some(artist) = &track.metadata.artist {
                if let Some(album) = &track.metadata.album {
                    return Some(format!("{} - {} \t | {}", artist, title, album));
                } else {
                    return Some(format!("{} - {}", artist, title));
                }
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
        self.engine.reset_sink();

        self.queue.next_track();

        self.play_current()
    }

    pub fn previous_track(&mut self) -> Result<()> {
        self.engine.reset_sink();

        self.queue.previous_track();

        self.play_current()
    }

    pub fn pause(&mut self) {
        if matches!(self.state, PlaybackState::Playing) {
            self.engine.pause();
            self.state = PlaybackState::Paused;
        }
    }

    pub fn resume(&mut self) {
        self.engine.play();
        self.state = PlaybackState::Playing;
    }

    pub fn stop(&mut self) {
        self.engine.reset_sink();
        self.state = PlaybackState::Stopped;
    }

    pub fn set_volume(&self, volume: f32) {
        self.engine.set_volume(volume);
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
