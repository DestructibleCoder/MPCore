use std::path::PathBuf;

#[derive(Debug)]
pub enum Command {
    Play(Option<usize>),
    Pause,
    Resume,
    Stop,
    Next,
    Prev,
    List,
    Seek(u64),
    Volume(f32),
    Playlist(PlaylistCmd),
    Exit,
}

#[derive(Debug)]
pub enum PlaylistCmd {
    Load(PathBuf),
    Save(PathBuf),
    Extend(PathBuf),
    RemoveTrack { path: PathBuf, index: usize },
    Delete(PathBuf),
}

impl Command {
    pub fn parse(input: &str) -> Option<Self> {
        let parts = shlex::split(input)?;
        if parts.is_empty() {
            return None;
        }

        match parts[0].as_str() {
            "play" => {
                let idx = parts.get(1).and_then(|s| s.parse().ok());
                Some(Command::Play(idx))
            }
            "pause" => Some(Command::Pause),
            "resume" => Some(Command::Resume),
            "stop" => Some(Command::Stop),
            "next" => Some(Command::Next),
            "seek" => {
                let secs = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                Some(Command::Seek(secs))
            }
            "prev" => Some(Command::Prev),
            "list" => Some(Command::List),
            "vol" => {
                let vol = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(1.0);
                Some(Command::Volume(vol))
            }
            "exit" | "quit" => Some(Command::Exit),

            "playlist" => {
                let sub_cmd = parts.get(1)?.as_str();
                let path = PathBuf::from(parts.get(2)?);

                let cmd = match sub_cmd {
                    "load" => PlaylistCmd::Load(path),
                    "save" => PlaylistCmd::Save(path),
                    "extend" => PlaylistCmd::Extend(path),
                    "del" => PlaylistCmd::Delete(path),
                    "remove_track" => {
                        let index = parts.get(3)?.parse().ok()?;
                        PlaylistCmd::RemoveTrack { path, index }
                    }
                    _ => return None,
                };
                Some(Command::Playlist(cmd))
            }
            _ => None,
        }
    }
}
