use corelib::player::Player;
use rustyline::highlight::Highlighter;
use std::borrow::Cow;
use std::sync::{Arc, Mutex};

use corelib::player::PlaybackState;

#[derive(rustyline::Helper, rustyline::Completer, rustyline::Hinter, rustyline::Validator)]
pub struct CliHelper {
    pub player: Arc<Mutex<Player>>,
}

impl Highlighter for CliHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            let state = self.player.lock().unwrap().state();
            match state {
                PlaybackState::Playing => Cow::Owned(format!("\x1b[32m{}\x1b[0m", prompt)), // Зеленый
                PlaybackState::Paused => Cow::Owned(format!("\x1b[33m{}\x1b[0m", prompt)), // Желтый
                PlaybackState::Stopped => Cow::Owned(format!("\x1b[31m{}\x1b[0m", prompt)), // Красный
            }
        } else {
            Cow::Borrowed(prompt)
        }
    }

    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        Cow::Borrowed(line)
    }
}
