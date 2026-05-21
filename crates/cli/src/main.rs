use anyhow::Result;
use colored::*;
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::time::Duration;

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

fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();

    let minutes = secs / 60;

    let seconds = secs % 60;

    format!("{:02}:{:02}", minutes, seconds)
}

fn render_progress_bar(current: Duration, total: Duration) -> String {
    let width = 20;

    let progress = current.as_secs_f32() / total.as_secs_f32();

    let filled = (progress * width as f32) as usize;

    let empty = width - filled;

    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let queue = if args.len() == 2 {
        Queue::load_from_folder(Path::new(&args[1]))?
    } else {
        Queue::load_from_folder(Path::new("./music"))?
    };

    let mut player = Player::new(queue)?;

    println!("Welcome to MPCore CLI");

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
                println!("====================\n");
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

            "status" => {
                if let Some((current, total)) = player.progress() {
                    println!(
                        "{} {} / {}",
                        render_progress_bar(current, total),
                        format_duration(current),
                        format_duration(total)
                    );
                }
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
