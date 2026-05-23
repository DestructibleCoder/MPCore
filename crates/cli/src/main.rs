use anyhow::Result;
use colored::*;
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use corelib::player::{PlaybackState, Player, RepeatMode};
use corelib::queue::Queue;

fn render_prompt(state: PlaybackState) -> String {
    match state {
        PlaybackState::Playing => {
            format!("{} > ", "▶ playing".green())
        }
        PlaybackState::Paused => {
            format!("{} > ", "⏸ paused".yellow())
        }
        PlaybackState::Stopped => {
            format!("{} > ", "■ stopped".red())
        }
    }
}

fn format_duration_optional(duration: Option<Duration>) -> String {
    match duration {
        Some(d) => {
            let secs = d.as_secs();

            let minutes = secs / 60;

            let seconds = secs % 60;

            format!("{:02}:{:02}", minutes, seconds)
        }

        None => String::from("--:--"),
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let queue = if args.len() == 2 {
        Queue::load_from_folder(Path::new(&args[1]))?
    } else {
        Queue::load_from_folder(Path::new("./music"))?
    };

    let player = Arc::new(Mutex::new(Player::new(queue)?));

    let autoplay_player = Arc::clone(&player);

    thread::spawn(move || {
        loop {
            {
                let mut player = autoplay_player.lock().unwrap();

                player.update().unwrap();
            }

            thread::sleep(Duration::from_millis(500));
        }
    });

    println!("Welcome to MPCore CLI");

    loop {
        print!("{}", render_prompt(player.lock().unwrap().state()));
        io::stdout().flush()?;

        let mut input = String::new();

        io::stdin().read_line(&mut input)?;

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "play" => {
                if parts.len() == 2 {
                    match parts[1].parse::<usize>() {
                        Ok(index) => {
                            player.lock().unwrap().play_track(index)?;
                        }
                        Err(_) => {
                            println!("Index out of range");
                        }
                    }
                } else {
                    player.lock().unwrap().play_current()?;
                }
            }

            "help" => {
                println!("Commands:\n");
                println!("====================");
                println!("======= play =======");
                println!("======= pause ======");
                println!("======= stop =======");
                println!("======= next =======");
                println!("======= prev =======");
                println!("======= quit =======");
                println!("======= name =======");
                println!("======= list =======");
                println!("====== status ======");
                println!("=== play <index> ===");
                println!("= shuffle <on|off> =");
                println!("==== load <file> ===");
                println!("==== save <file> ===");
                println!("=== repeat <mode> ==");
                println!("== seek <seconds> ==");
                println!("====================\n");
            }

            "save" => {
                if parts.len() != 2 {
                    println!("save <file>");
                    continue;
                }

                player.lock().unwrap().save_playlist(Path::new(parts[1]))?;

                println!("Playlist saved in {}!", parts[1]);
            }

            "load" => {
                if parts.len() != 2 {
                    println!("load <file>");
                    continue;
                }

                player.lock().unwrap().load_playlist(Path::new(parts[1]))?;

                println!("Playlist {} loaded!", parts[1]);
            }

            "repeat" => {
                if parts.len() != 2 {
                    println!("repeat <none|queue|track>");
                    continue;
                }

                match parts[1] {
                    "none" => {
                        player.lock().unwrap().set_repeat_mode(RepeatMode::None);
                    }

                    "queue" => {
                        player.lock().unwrap().set_repeat_mode(RepeatMode::Queue);
                    }

                    "track" => {
                        player.lock().unwrap().set_repeat_mode(RepeatMode::Track);
                    }

                    _ => {
                        println!("Unknown repeat mode!");
                    }
                }
            }

            "shuffle" => {
                if parts.len() != 2 {
                    println!("shuffle <on|off>");
                    continue;
                }

                match parts[1] {
                    "on" => {
                        player.lock().unwrap().shuffle();
                        println!("shuffle enabled");
                    }

                    "off" => {
                        player.lock().unwrap().unshuffle();
                        println!("shuffle disabled");
                    }

                    _ => {
                        println!("Unknown shuffle mode!");
                    }
                }
            }

            "pause" => {
                player.lock().unwrap().pause();
            }

            "stop" => {
                player.lock().unwrap().stop();
            }

            "next" => {
                player.lock().unwrap().next_track()?;
            }

            "prev" => {
                player.lock().unwrap().previous_track()?;
            }

            "vol" => {
                if parts.len() < 2 {
                    println!("Usage: vol <number>");
                    continue;
                }

                let volume: f32 = match parts[1].parse() {
                    Ok(v) => v,
                    Err(_) => {
                        println!("Invalid volume!");
                        continue;
                    }
                };

                player.lock().unwrap().set_volume(volume);

                println!("volume set to {}", volume);
            }

            "seek" => {
                if parts.len() != 2 {
                    println!("seek <seconds>");
                    continue;
                }

                let Ok(seconds) = parts[1].parse::<u64>() else {
                    println!("Invalid number");
                    continue;
                };

                player.lock().unwrap().seek(seconds)?;
            }

            "list" => {
                for (i, track) in player.lock().unwrap().queue().tracks.iter().enumerate() {
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

                    println!(
                        "{:>2}. {:<80} {}",
                        i,
                        name,
                        format_duration_optional(track.duration)
                    );
                }
            }

            "name" => match player.lock().unwrap().get_track_info() {
                Some(track_info) => println!("{}", track_info),
                None => println!("No tracks selected"),
            },

            "quit" => {
                println!("Bye!");
                break;
            }

            _ => {
                println!("Unknown command");
            }
        }
    }

    Ok(())
}
