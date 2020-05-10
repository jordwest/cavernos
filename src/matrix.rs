use crate::cavernos;
use rand::prelude::*;

struct Particle {
    x: u8,
    y: f64,
    speed: f64,
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
            y: 0.0,
            speed: self.rng.gen_range(13.0, 70.0),
            index: self.rng.gen_range(0, 31),
        });
    }
}

pub fn frame(os: &mut cavernos::CavernOS, matrix_state: &mut MatrixState, dt: f64) {
    matrix_state.t += dt;

    if matrix_state.t < 0.03 {
        return;
    }
    matrix_state.t = 0.0;

    let cols = os.config.cols;

    os.screen.update = true;
    os.bg_color.update = true;
    os.fg_color.update = true;

    matrix_state.add_particle(cols);
    matrix_state.add_particle(cols);

    for particle in &mut matrix_state.particles {
        let x = particle.x;
        let y = particle.y;
        particle.index = matrix_state.rng.gen_range(0, 31);
        let i: usize = (y as usize) * (cols as usize) + (x as usize);
        os.screen.cells[i] = 224 + particle.index;

        let cell_y = particle.y.floor() as u8;
        for dist in 0..=cell_y {
            let i: usize = ((cell_y - dist) as usize) * (cols as usize) + (x as usize);

            let col = if (dist / 2) < 16 { 64 + (dist / 2) } else { 0 };
            os.fg_color.cells[i] = col as u8;
            os.bg_color.cells[i] = 0;
        }

        particle.y += dt * particle.speed;
    }

    matrix_state.particles.retain(|p| p.y <= (cols as f64));

    let mut i = 61;
    for c in "You are the one".chars() {
        i += 1;
        os.screen.cells[i] = c as u8;
    }
}
