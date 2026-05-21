use crate::track::Track;

use anyhow::Result;

use std::fs;
use std::fs::File;
use std::path::Path;

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use rodio::{Decoder, Source};

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

            if !path.is_file() {
                continue;
            }

            let Some(extension) = path.extension() else {
                continue;
            };

            let extension = extension.to_string_lossy().to_lowercase();

            let supported = matches!(extension.as_str(), "mp3" | "flac" | "wav" | "ogg");

            if supported {
                let duration = File::open(&path)
                    .ok()
                    .and_then(|file| Decoder::try_from(file).ok())
                    .and_then(|decoder| decoder.total_duration());

                tracks.push(Track { path, duration });
            }
        }

        Ok(Self {
            tracks: tracks.clone(),
            original_tracks: tracks,
            current: 0,
        })
    }

    pub fn save_playlist(&self, path: &Path) -> Result<()> {
        let file = File::create(path)?;

        serde_json::to_writer_pretty(file, &self.tracks)?;

        Ok(())
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
}
