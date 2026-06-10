use crate::playlist::Playlist;
use crate::track::Track;

use anyhow::Result;

use std::fs;
use std::fs::File;
use std::path::Path;

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Queue {
    tracks: Vec<Track>,
    #[serde(skip)]
    order: Vec<usize>,
    current: Option<usize>,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
            order: Vec::new(),
            current: None,
        }
    }

    pub fn load_from_folder(path: &Path) -> Result<Self> {
        let mut tracks = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;

            let path = entry.path();

            if let Ok(track) = Track::from_path(path) {
                tracks.push(track);
            }
        }

        let len = tracks.len();

        Ok(Self {
            tracks: tracks,
            order: (0..len).collect(),
            current: if len > 0 { Some(0) } else { None },
        })
    }

    pub fn next_track(&mut self) {
        let Some(current) = self.current else {
            return;
        };

        if self.order.is_empty() {
            return;
        }

        self.current = Some((current + 1) % self.order.len())
    }

    pub fn load_playlist(path: &Path) -> Result<Self> {
        let playlist: Playlist = Playlist::load(path)?;

        let len = playlist.tracks.len();

        Ok(Self {
            tracks: playlist.tracks,
            order: (0..len).collect(),
            current: if len > 0 { Some(0) } else { None },
        })
    }

    pub fn save_playlist(&self, path: &Path) -> Result<()> {
        let Some(name) = path.file_stem().and_then(|name| name.to_str()) else {
            anyhow::bail!("Bad filename!");
        };
        let playlist: Playlist = Playlist::new(name.to_string(), self.tracks.clone());

        playlist.save(path)?;

        Ok(())
    }

    pub fn add_from_playlist(&mut self, path: &Path) -> Result<()> {
        let file = File::open(path)?;

        let new_tracks: Vec<Track> = serde_json::from_reader(file)?;

        self.tracks.extend(new_tracks);

        Ok(())
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();

        self.order.shuffle(&mut rng);

        self.current = Some(0);
    }

    pub fn unshuffle(&mut self) {
        // WARN: FUNCTION IN BUGFIX
        // self.order = (0..self.tracks.len()).collect();
    }

    pub fn previous_track(&mut self) {
        let Some(current) = self.current else {
            return;
        };

        if self.order.is_empty() {
            return;
        }

        self.current = Some(if current == 0 {
            self.order.len() - 1
        } else {
            current - 1
        })
    }

    pub fn set_current(&mut self, index: usize) {
        if index < self.order.len() {
            self.current = Some(index);
        }
    }

    pub fn order(&self) -> &Vec<usize> {
        &self.order
    }

    pub fn load_track_from_path(&mut self, path: std::path::PathBuf) -> Result<()> {
        let track = Track::from_path(path)?;

        let idx = self.tracks.len();

        self.tracks.push(track);
        self.order.push(idx);

        if self.current.is_none() {
            self.current = Some(0);
        }

        Ok(())
    }

    pub fn clear(&mut self) {
        self.tracks.clear();
        self.order.clear();
        self.current = None;
    }

    pub fn get_track(&self, index: usize) -> Option<&Track> {
        let real_index = *self.order.get(index)?;

        self.tracks.get(real_index)
    }

    pub fn current_track(&self) -> Option<&Track> {
        let current = self.current?;

        self.get_track(current)
    }

    pub fn len(&self) -> usize {
        self.tracks.len()
    }

    pub fn current(&self) -> Option<usize> {
        self.current
    }

    pub fn tracks(&self) -> &[Track] {
        &self.tracks
    }
}
