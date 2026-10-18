mod engine;
mod fireworks;

use crate::{
  engine::Scene,
  fireworks::{FireworksConfig, FireworksScene},
};
use clap::{Parser, Subcommand};
use crossterm::{
  self,
  cursor::{Hide, Show},
  execute,
  terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, stdout};

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

#[derive(Parser)]
#[command(version, about = "A collection of simple console toys written in Rust", long_about = None)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  /// Shoot fireworks in your terminal
  Fireworks {
    /// Use <Enter> to manually shoot fireworks
    #[arg(short, long)]
    manual: bool,

    /// Use more than 6 colors
    #[arg(short, long)]
    true_random: bool,
  },
}

fn main() -> io::Result<()> {
  let cli = Cli::parse();
  match &cli.command {
    Commands::Fireworks {
      manual,
      true_random,
    } => {
      let _guard = TermGuard::new();
      let mut scene = FireworksScene::new(FireworksConfig::new(*manual, *true_random))?;
      scene.run()
    }
  }
}
