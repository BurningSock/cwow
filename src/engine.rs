pub mod buffer;

use buffer::Buffer;
use crossterm::{
  execute,
  terminal::{Clear, ClearType, size},
};
use std::{io, time::Duration};

pub enum Commands {
  Update,
  AddEntity(Box<dyn Entity>),
}

pub trait Entity {
  fn update(&mut self, delta: f64) -> Option<Vec<Commands>>;
  fn draw(&self, buffer: &mut Buffer);
}

pub trait Scene {
  fn context(&mut self) -> &mut SceneContext;

  fn run(&mut self) -> io::Result<()>;

  fn update(&mut self, delta: f64) {
    self.context().update_entities(delta);
  }
}

pub struct SceneContext {
  crt: Buffer,
  next: Buffer,
  changed: bool,
  entities: Vec<Box<dyn Entity>>,
}

impl SceneContext {
  pub fn new() -> io::Result<Self> {
    let (cols, rows) = size()?;
    Ok(Self {
      crt: Buffer::new((cols, rows)),
      next: Buffer::new((cols, rows)),
      changed: false,
      entities: Vec::new(),
    })
  }
  pub fn size(&self) -> (u16, u16) {
    (self.crt.cols(), self.crt.rows())
  }

  pub fn execute_cmd(&mut self, cmd: Commands) {
    self.changed = true;
    match cmd {
      Commands::AddEntity(entity) => self.entities.push(entity),
      Commands::Update => (),
    }
  }
  pub fn update_entities(&mut self, delta: f64) {
    let mut pending_commands = Vec::new();

    for e in &mut self.entities {
      if let Some(cmds) = e.update(delta) {
        pending_commands.extend(cmds);
      }
    }

    for cmd in pending_commands {
      self.execute_cmd(cmd);
    }
  }
  pub fn render(&mut self) -> io::Result<()> {
    let (cols, rows) = size()?;
    let resized = (cols, rows) != self.size();

    if !resized && !self.changed {
      return Ok(());
    }

    if resized {
      execute!(io::stdout(), Clear(ClearType::All))?;
      self.crt = Buffer::new((cols, rows));
      self.next = Buffer::new((cols, rows));
    }

    self.next.clear();
    for e in &self.entities {
      e.draw(&mut self.next);
    }
    self.next.print(&self.crt)?;
    std::mem::swap(&mut self.crt, &mut self.next);
    Ok(())
  }
}

pub const REFRESH_RATE: Duration = Duration::from_nanos(1_000_000_000 / 60); // 60 Hz
