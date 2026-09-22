pub mod buffer;

use buffer::Buffer;
use crossterm::{
  execute,
  terminal::{Clear, ClearType, size},
};
use std::{
  io::{self, stdout},
  time::Duration,
};

pub trait Entity {
  fn update(&mut self, delta: f64);
  fn draw(&self, buffer: &mut Buffer);
}

pub struct Scene {
  crt: Buffer,
  next: Buffer,
  objs: Vec<Box<dyn Entity>>,
}
impl Scene {
  pub fn new() -> io::Result<Self> {
    let size = size()?;
    Ok(Self {
      crt: Buffer::new(size),
      next: Buffer::new(size),
      objs: Vec::new(),
    })
  }

  pub fn append_objs(&mut self, mut objs: Vec<Box<dyn Entity>>) {
    self.objs.append(&mut objs);
  }

  pub fn size(&self) -> (u16, u16) {
    (self.crt.cols(), self.crt.rows())
  }

  pub fn render(&mut self) -> io::Result<()> {
    let (cols, rows) = size()?;

    let resized = self.crt.cols() != cols || self.crt.rows() != rows;

    if resized {
      execute!(stdout(), Clear(ClearType::All))?;
      self.crt = Buffer::new((cols, rows));
      self.next = Buffer::new((cols, rows));
    } else {
      self.next.clear();
    }

    for obj in self.objs.iter_mut() {
      obj.draw(&mut self.next);
    }

    self.next.print(&self.crt)?;
    std::mem::swap(&mut self.crt, &mut self.next);

    Ok(())
  }

  pub fn update(&mut self, delta: f64) {
    for obj in self.objs.iter_mut() {
      obj.update(delta);
    }
  }
}

pub const REFRESH_RATE: Duration = Duration::from_nanos(1_000_000_000 / 60); //60 Hz
