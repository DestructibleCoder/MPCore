use anyhow::Result;
use colored::*;
use std::io::{self, Write};
use std::path::Path;

use corelib::player::{PlaybackState, Player};
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

fn main() -> Result<()> {
    let queue = Queue::load_from_folder(Path::new("./music"))?;

    let mut player = Player::new(queue)?;

    println!("Welcome to MPCore CLI");
    println!("Commands:\n");
    println!("====================");
    println!("======= play =======");
    println!("======= pause ======");
    println!("======= stop =======");
    println!("======= next =======");
    println!("======= prev =======");
    println!("======= quit =======");
    println!("====================\n");

    loop {
        print!("{}", render_prompt(player.state()));
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
                            player.play_track(index)?;
                        }
                        Err(_) => {
                            println!("Index out of range");
                        }
                    }
                } else {
                    player.play_current()?;
                }
            }

            "pause" => {
                player.pause();
            }

            "stop" => {
                player.stop();
            }

            "next" => {
                player.next_track()?;
            }

            "prev" => {
                player.previous_track()?;
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

                player.set_volume(volume);

                println!("volume set to {}", volume);
            }

            "list" => {
                for (i, track) in player.queue().tracks.iter().enumerate() {
                    let name = track
                        .path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();

                    println!("{:>2}. {}", i, name);
                }
            }

            "name" => match player.current_track_name() {
                Some(name) => println!("{}", name),
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
