use corelib::queue::Queue;

use anyhow::Result;

use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    let music_path = Path::new("./music");

    if !music_path.exists() {
        println!("");

        println!("");

        fs::create_dir_all(music_path)?;
    }

    let queue = Queue::load_from_folder(music_path)?;

    println!("Loaded {} tracks", queue.tracks.len());

    for track in queue.tracks {
        println!("{:?}", track.path);
    }

    Ok(())
}
