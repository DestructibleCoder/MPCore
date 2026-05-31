use crate::track::Track;

use anyhow::Result;

use std::fs;
use std::fs::File;
use std::path::Path;

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Queue {
    pub tracks: Vec<Track>,
    #[serde(skip)]
    original_tracks: Vec<Track>,
    pub current: usize,
}

impl Queue {
    pub fn load_from_folder(path: &Path) -> Result<Self> {
        let mut tracks = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;

            let path = entry.path();

            if let Ok(track) = Track::from_path(path) {
                tracks.push(track);
            }
        }

        Ok(Self {
            tracks: tracks.clone(),
            original_tracks: tracks,
            current: 0,
        })
    }

    pub fn next_track(&mut self) {
        if self.tracks.is_empty() {
            return;
        }

        self.current = (self.current + 1) % self.tracks.len();
    }

    pub fn load_playlist(path: &Path) -> Result<Self> {
        let file = File::open(path)?;

        let tracks: Vec<Track> = serde_json::from_reader(file)?;

        Ok(Self {
            original_tracks: tracks.clone(),
            tracks,
            current: 0,
        })
    }

    pub fn add_from_playlist(&mut self, path: &Path) -> Result<()> {
        let file = File::open(path)?;

        let new_tracks: Vec<Track> = serde_json::from_reader(file)?;

        self.tracks.extend(new_tracks);

        Ok(())
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();

        self.tracks.shuffle(&mut rng);

        self.current = 0;
    }

    pub fn unshuffle(&mut self) {
        self.tracks = self.original_tracks.clone();

        self.current = 0;
    }

    pub fn previous_track(&mut self) {
        if self.tracks.is_empty() {
            return;
        }

        self.current = if self.current == 0 {
            self.tracks.len() - 1
        } else {
            self.current - 1
        };
    }

    pub fn set_current(&mut self, index: usize) {
        if index < self.tracks.len() {
            self.current = index;
        }
    }

    pub fn load_track_from_path(&mut self, path: std::path::PathBuf) -> Result<()> {
        if let Ok(track) = Track::from_path(path) {
            self.tracks.push(track);
        }

        Ok(())
    }

    pub fn clear(&mut self) {
        self.tracks.clear();
    }
}
