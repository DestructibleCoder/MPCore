use crate::{queue::Queue, track::Track};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

fn ensure_json_extension(path: &Path) -> PathBuf {
    let mut path = path.to_path_buf();

    path.set_extension("json");

    path
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub name: String,
    pub tracks: Vec<Track>,
}

impl Playlist {
    pub fn new(name: String, tracks: Vec<Track>) -> Self {
        Self { name, tracks }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;

        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Can't detect home dir!"))?;

        let full_path = home.join(path);

        fs::write(ensure_json_extension(&full_path), json)?;

        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self> {
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Can't detect home dir!"))?;

        let full_path = home.join(path);

        let file = File::open(ensure_json_extension(&full_path))?;

        let playlist = serde_json::from_reader(file)?;

        Ok(playlist)
    }

    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }

    pub fn remove_track(&mut self, index: usize) -> Option<Track> {
        if index < self.tracks.len() {
            Some(self.tracks.remove(index))
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.tracks.clear();
    }

    pub fn delete_playlist(path: &Path) -> Result<()> {
        fs::remove_file(ensure_json_extension(path))?;

        Ok(())
    }

    pub fn add_tracks_from_queue(&mut self, queue: &Queue) {
        let add = queue.tracks();
        self.tracks.extend(add.iter().cloned());
    }

    pub fn add_tracks_from_folder(&mut self, path: &Path) -> Result<()> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;

            let path = entry.path();

            if let Ok(track) = Track::from_path(path) {
                self.tracks.push(track);
            }
        }

        Ok(())
    }
}
