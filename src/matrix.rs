use crate::cavernos;
use rand::prelude::*;

fn map_range(src_a: f32, src_b: f32, dst_a: f32, dst_b: f32, val: f32) -> f32 {
  let src_range = src_b - src_a;
  let dst_range = dst_b - dst_a;

  let proportion = (val - src_a) / src_range;
  dst_a + (proportion * dst_range)
}

static mut FRAME_NUMBER: usize = 0;

struct Particle {
  x: u8,
  y: u8,
  index: u8,
}

pub struct MatrixState {
  t: f64,
  particles: Vec<Particle>,
  rng: rand::rngs::StdRng,
}
impl MatrixState {
  pub fn new() -> Self {
    MatrixState {
      t: 0.0,
      particles: Vec::new(),
      rng: rand::rngs::StdRng::seed_from_u64(983798534),
    }
  }

  pub fn add_particle(&mut self, cols: u8) {
    self.particles.push(Particle {
      x: self.rng.gen_range(0, cols),
      y: 0,
      index: self.rng.gen_range(0, 31),
    });
  }
}

pub fn frame(os: &mut cavernos::CavernOS, matrixState: &mut MatrixState, dt: f64) {
  matrixState.t += dt;

  if (matrixState.t < 0.03) {
    return;
  }
  matrixState.t = 0.0;

  let cols = os.config.cols;
  let rows = os.config.rows;

  os.screen.update = true;
  os.bg_color.update = true;
  os.fg_color.update = true;

  matrixState.add_particle(cols);
  matrixState.add_particle(cols);

  for particle in &mut matrixState.particles {
    let x = particle.x;
    let y = particle.y;
    particle.index = matrixState.rng.gen_range(0, 31);
    let i: usize = (y as usize) * (cols as usize) + (x as usize);
    os.screen.cells[i] = 224 + particle.index;

    for dist in 0..=particle.y {
      let i: usize = ((particle.y - dist) as usize) * (cols as usize) + (x as usize);

      let col = if ((dist / 2) < 16) {
        (64 + (dist / 2))
      } else {
        0
      };
      os.fg_color.cells[i] = col as u8;
    }

    particle.y += 1;
  }

  matrixState.particles.retain(|p| p.y <= cols);

  let mut i = 61;
  for c in "You are the one".chars() {
    i += 1;
    os.screen.cells[i] = c as u8;
  }
}
