use crossterm::{
  cursor::MoveTo,
  queue,
  style::{Color, PrintStyledContent, Stylize},
};
use std::{
  char,
  io::{self, Write},
};

#[derive(Clone, Copy, PartialEq)]
struct Cell {
  c: char,
  fg: Color,
  bg: Color,
}
impl Cell {
  fn empty() -> Self {
    Self {
      c: ' ',
      fg: crossterm::style::Color::Reset,
      bg: crossterm::style::Color::Reset,
    }
  }
}

pub struct Buffer {
  data: Vec<Cell>,
  cols: u16,
  rows: u16,
}
impl Buffer {
  pub fn new(size: (u16, u16)) -> Self {
    Self {
      data: vec![Cell::empty(); usize::from(size.0) * usize::from(size.1)],
      cols: size.0,
      rows: size.1,
    }
  }

  pub fn clear(&mut self) {
    self.data.fill(Cell::empty());
  }

  pub fn cols(&self) -> u16 {
    self.cols
  }

  pub fn rows(&self) -> u16 {
    self.rows
  }

  pub fn place_at_pos(&mut self, col: u16, row: u16, c: char, fg: Color) {
    let index = self.get_index((col, row));
    self.data[index].c = c;
    self.data[index].fg = fg;
  }

  pub fn get_index(&self, pos: (u16, u16)) -> usize {
    let (col, row) = (pos.0 as usize, pos.1 as usize);
    let cols = self.cols as usize;

    row.saturating_mul(cols).saturating_add(col)
  }

  pub fn print(&self, old: &Self) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    let old_max_index = (old.cols as usize).saturating_mul(old.rows as usize);

    for row in 0..self.rows {
      for col in 0..self.cols {
        let index = self.get_index((col, row));
        let cell = &self.data[index];

        let changed = old
          .data
          .get(index)
          .map_or(true, |old_cell| *old_cell != *cell);
        let out_of_bounds = index >= old_max_index;

        if out_of_bounds || changed {
          queue!(
            stdout,
            MoveTo(col, row),
            PrintStyledContent(cell.c.with(cell.fg).on(cell.bg))
          )?;
        }
      }
    }

    stdout.flush()?;

    Ok(())
  }
}
