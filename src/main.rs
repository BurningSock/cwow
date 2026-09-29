mod engine;
mod fireworks;

use crossterm::{
  self,
  cursor::{Hide, Show},
  execute,
  terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, stdout};

use crate::fireworks::FireworksConfig;

struct TermGuard;
impl TermGuard {
  fn new() -> io::Result<Self> {
    crossterm::terminal::enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, Hide)?;
    Ok(Self)
  }
}
impl Drop for TermGuard {
  fn drop(&mut self) {
    if let Err(e) = execute!(stdout(), LeaveAlternateScreen, Show) {
      eprintln!("warning: unable to reset the screen: {e}");
    }
    if let Err(e) = crossterm::terminal::disable_raw_mode() {
      eprintln!("warning: unable to disable raw mode: {e}");
    }
  }
}

fn main() -> io::Result<()> {
  run()
}

fn run() -> io::Result<()> {
  let _guard = TermGuard::new();
  fireworks::run(FireworksConfig::new(true, true))
}
