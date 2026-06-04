use crate::commands::{Command, PlaylistCmd};
use anyhow::Result;
use corelib::player::Player;
use corelib::playlist::Playlist;
use std::sync::{Arc, Mutex};

fn cmd_list(player: &Arc<Mutex<Player>>) -> Result<()> {
    let p = player.lock().unwrap();
    let tracks = p.queue().tracks();

    if tracks.is_empty() {
        println!("Playback queue is empty!");
        return Ok(());
    }

    for (i, track) in tracks.iter().enumerate() {
        let name = if let Some(title) = &track.metadata.title {
            if let Some(artist) = &track.metadata.artist {
                format!("{} - {}", artist, title)
            } else {
                title.clone()
            }
        } else {
            track
                .path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        };

        let first_column = format!("{}. {}", i, name);

        let display_name = if first_column.chars().count() > 65 {
            let mut truncated: String = first_column.chars().take(62).collect();
            truncated.push_str("...");
            truncated
        } else {
            format!("{:<65}", first_column)
        };

        let album = track.metadata.album.as_deref().unwrap_or("Unknown Album");

        println!("{} | {}", display_name, album);
    }
    Ok(())
}

pub fn execute_command(cmd: Command, player: &Arc<Mutex<Player>>) -> Result<bool> {
    match cmd {
        Command::Play(Some(idx)) => {
            player.lock().unwrap().play_track(idx)?;
        }
        Command::Play(None) => player.lock().unwrap().resume(),
        Command::Pause => player.lock().unwrap().pause(),
        Command::Resume => player.lock().unwrap().resume(),
        Command::Stop => player.lock().unwrap().stop(),
        Command::Next => player.lock().unwrap().next_track()?,
        Command::Prev => player.lock().unwrap().previous_track()?,

        Command::Seek(seconds) => player.lock().unwrap().seek(seconds)?,

        Command::List => cmd_list(player)?,
        Command::Volume(vol) => player.lock().unwrap().set_volume(vol),

        Command::Playlist(sub_cmd) => match sub_cmd {
            PlaylistCmd::Load(path) => {
                player.lock().unwrap().load_playlist(&path)?;
                println!("Playlist loaded.");
            }
            PlaylistCmd::Save(path) => {
                player.lock().unwrap().save_playlist(&path)?;
                println!("Playlist saved.");
            }
            PlaylistCmd::Extend(path) => {
                player.lock().unwrap().extend_playlist(&path)?;
            }
            PlaylistCmd::Delete(path) => {
                Playlist::delete_playlist(&path)?;
                println!("Playlist deleted.");
            }
            PlaylistCmd::RemoveTrack { path, index } => {
                let mut playlist = Playlist::load(&path)?;
                if let Some(track) = playlist.remove_track(index) {
                    println!("Track {:?} deleted.", track.metadata.title);
                    playlist.save(&path)?;
                } else {
                    anyhow::bail!("Index out of range!");
                }
            }
        },

        Command::Exit => return Ok(true),
    }
    Ok(false)
}
