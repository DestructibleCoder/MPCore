mod commands;
mod handler;
mod prompt;

use anyhow::Result;
use std::sync::{Arc, Mutex};

use rustyline::Editor;
use rustyline::history::DefaultHistory;

use commands::Command;
use corelib::player::{PlaybackState, Player};
use corelib::queue::Queue;
use prompt::CliHelper;

fn get_plain_prompt(state: PlaybackState) -> &'static str {
    match state {
        PlaybackState::Playing => "▶ playing > ",
        PlaybackState::Paused => "⏸ paused > ",
        PlaybackState::Stopped => "■ stopped > ",
    }
}

fn main() -> Result<()> {
    let music_dir = dirs::audio_dir().unwrap_or_else(|| std::path::PathBuf::from("./music"));

    let queue = Queue::load_from_folder(&music_dir).unwrap_or_else(|_| Queue::new());
    let player = Arc::new(Mutex::new(Player::new(queue)?));

    let mut rl: Editor<CliHelper, DefaultHistory> = Editor::new()?;

    let helper = CliHelper {
        player: Arc::clone(&player),
    };
    rl.set_helper(Some(helper));
    let _ = rl.load_history(".mpcore_history");

    println!("Welcome to MpCore CLI!");

    loop {
        let state = player.lock().unwrap().state();
        let prompt = get_plain_prompt(state);

        match rl.readline(prompt) {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }

                let _ = rl.add_history_entry(input);

                if let Some(cmd) = Command::parse(input) {
                    match handler::execute_command(cmd, &player) {
                        Ok(should_exit) => {
                            if should_exit {
                                break;
                            }
                        }
                        Err(e) => println!("Error: {}", e),
                    }
                } else {
                    println!("Unknown command or unknown args.");
                }
            }
            Err(_) => break,
        }
    }

    let _ = rl.save_history(".mpcore_history");
    println!("Bye Bye! :3");
    Ok(())
}
