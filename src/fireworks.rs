use crate::engine::{self, Entity, Scene, buffer::Buffer};
use crossterm::{
  event::{self, Event, KeyCode},
  style::Color,
};
use rand::random_range;
use std::{
  f64::consts::TAU,
  io,
  ops::{Add, Mul},
  thread::sleep,
  time::{self, Duration},
};

pub struct FireworksConfig {
  manual: bool,
  true_random: bool,
}
impl FireworksConfig {
  pub fn new(manual: bool, true_random: bool) -> Self {
    Self {
      manual,
      true_random,
    }
  }
}

pub struct Firework {
  pos: (u16, u16),
  color: crossterm::style::Color,
}
impl Firework {
  pub fn new(pos: (u16, u16), color: crossterm::style::Color) -> Self {
    Firework { pos, color }
  }

  pub fn random_color(&mut self) {
    const COLORS: [Color; 6] = [
      Color::Red,
      Color::Green,
      Color::Yellow,
      Color::Blue,
      Color::Magenta,
      Color::Cyan,
    ];

    self.color = COLORS[random_range(0..COLORS.len())];
  }
  pub fn true_random_color(&mut self) {
    self.color = Color::Rgb {
      r: random_range(100..=255),
      g: random_range(100..=255),
      b: random_range(100..=255),
    };
  }

  pub fn explode(self) -> Vec<Box<dyn Entity>> {
    let amount = (random_range(20..40)) as usize;
    let mut entities: Vec<Box<dyn Entity>> = Vec::with_capacity(amount);

    for i in 0..amount {
      let speed = random_range(80.0..100.0);
      let point = Point {
        lifetime: random_range(1.0..2.0),
        color: self.color,
        position: (self.pos.0 as f64, self.pos.1 as f64),
        velocity: (
          (i as f64 / amount as f64 * TAU).cos() * speed,
          (i as f64 / amount as f64 * TAU).sin() * speed * 0.5,
        ),
        acceleration: (0.0, 9.81 * 10.0),
      };

      entities.push(Box::new(point));
    }

    entities
  }
}

struct Point {
  lifetime: f64,
  color: crossterm::style::Color,
  position: (f64, f64),
  velocity: (f64, f64),
  acceleration: (f64, f64),
}
impl Entity for Point {
  fn update(&mut self, delta: f64) {
    self.lifetime -= delta;
    self.velocity = (
      self.velocity.0.add(self.acceleration.0.mul(delta)),
      self.velocity.1.add(self.acceleration.1.mul(delta)),
    );

    self.position = (
      self.position.0.add(self.velocity.0.mul(delta)),
      self.position.1.add(self.velocity.1.mul(delta)),
    );
  }

  fn draw(&self, buffer: &mut Buffer) {
    let (col, row) = self.position;

    if col.is_sign_negative() || row.is_sign_negative() {
      return;
    }

    let col = (u16::MAX as f64).min(col) as u16;
    let row = (u16::MAX as f64).min(row) as u16;

    if col >= buffer.cols() || row >= buffer.rows() {
      return;
    }

    if self.lifetime <= 0.0 {
      buffer.place_at_pos(col, row, ' ', self.color);
    } else if self.lifetime < 1.0 {
      buffer.place_at_pos(col, row, '.', self.color);
    } else if self.lifetime < 1.85 {
      buffer.place_at_pos(col, row, '*', self.color);
    } else {
      buffer.place_at_pos(col, row, '@', self.color);
    }
  }
}

const SPAWN_TIME: Duration = Duration::from_millis(100);

pub fn run(fc: FireworksConfig) -> io::Result<()> {
  let mut last_frame = time::Instant::now();
  let mut scene = Scene::new()?;
  let mut timer = Duration::ZERO;

  'main: loop {
    let start = time::Instant::now();
    let last_frame_delta = start.duration_since(last_frame);
    last_frame = start;

    if !fc.manual {
      timer += last_frame_delta;
      while timer >= SPAWN_TIME {
        timer -= SPAWN_TIME;
        shoot_firework(&mut scene, &fc);
      }
    }

    if crossterm::event::poll(Duration::ZERO)? {
      if let Event::Key(key_event) = event::read()? {
        if key_event.is_press() {
          match key_event.code {
            KeyCode::Char('q') => break 'main,
            KeyCode::Enter => {
              if fc.manual {
                shoot_firework(&mut scene, &fc);
              }
            }
            _ => (),
          };
        }
      }
    }

    scene.update(last_frame_delta.as_secs_f64());
    scene.render()?;

    let delta = start.elapsed();
    if !engine::REFRESH_RATE.saturating_sub(delta).is_zero() {
      sleep(engine::REFRESH_RATE.saturating_sub(delta));
    }
  }

  Ok(())
}

fn shoot_firework(scene: &mut Scene, fc: &FireworksConfig) {
  let pos = (
    random_range(0..scene.size().0),
    random_range(0..scene.size().1),
  );
  let mut firework = Firework::new(pos, crossterm::style::Color::Reset);
  if fc.true_random {
    firework.true_random_color();
  } else {
    firework.random_color();
  }
  let entities = firework.explode();
  scene.append_objs(entities);
}
