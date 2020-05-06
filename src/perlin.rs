use crate::cavernos;
use noise::{NoiseFn, Seedable};

fn float_to_ascii_char(val: f32) -> u8 {
  let i = (val * 100.0) as i32;
  match i {
    -10..=10 => 0,
    -20..=20 => 2,
    -70..=70 => 3,
    -90..=90 => 14,
    -100..=100 => 177,
    -120..=120 => 178,
    _ => 2,
  }
}

fn map_range(src_a: f32, src_b: f32, dst_a: f32, dst_b: f32, val: f32) -> f32 {
  let src_range = src_b - src_a;
  let dst_range = dst_b - dst_a;

  let proportion = (val - src_a) / src_range;
  dst_a + (proportion * dst_range)
}

static mut FRAME_NUMBER: usize = 0;

pub fn draw(os: &mut cavernos::CavernOS, appState: &mut crate::state::AppState) {
  let mut t = 0.0;
  unsafe {
    if os.inputs[2] > 0 {
      FRAME_NUMBER -= 1;
    } else if os.inputs[3] > 0 {
      FRAME_NUMBER += 2;
    } else if os.inputs[5] > 0 {
      // Do nothing
    } else if FRAME_NUMBER == 0 {
      FRAME_NUMBER = 1;
    } else {
      return;
    }
    t = (FRAME_NUMBER as f32) / 60.0;
  }

  let cols = os.config.cols;
  let rows = os.config.rows;
  let n = noise::Perlin::new();
  let n = n.set_seed(18);
  let v = n.get([16.3, 32.0, 8.0]);
  //log(format!("{:?}", v));
  os.screen.update = true;
  os.bg_color.update = true;
  os.fg_color.update = true;

  for y in 0..rows {
    for x in 0..cols {
      let v = n.get([
        x as f64 / cols as f64 * 2.0,
        y as f64 / rows as f64 * 2.0,
        t as f64 / 8.0,
      ]);
      //let v = n.get([2.1, 4.6, 8.0]);
      //log(format!("{:?}", v));

      let i: usize = (y as usize) * (cols as usize) + (x as usize);
      let dx = (x as f32) - (cols as f32) / 2.0;
      let dy = (y as f32) - (rows as f32);

      let bg_color = map_range(-1.0, 1.0, 16.0, 48.0, v as f32) as u8;
      //let height = map_range(-1.0, 1.0, 48.0, 63.0, v as f32) as u8;
      let fg_color = map_range(-1.0, 1.0, 16.0, 48.0, 1.0 - v.abs() as f32) as u8;

      //let dist = (dx.powi(2) + dy.powi(2)).sqrt();
      //let t = ((FRAME_NUMBER as f32) + (dist / 10.0)) / 120.0;

      let fac_y = t.sin() * 3.0;

      if i % 2 == 0 {
        os.screen.cells[i] = 255;
        os.screen.cells[i + 1] = 249;
        os.screen.cells[i + 1] = float_to_ascii_char(v as f32);

        os.bg_color.cells[i] = bg_color;
        os.bg_color.cells[i + 1] = bg_color;
        os.fg_color.cells[i] = fg_color;
        os.fg_color.cells[i + 1] = fg_color;
      }
    }
  }

  //panic!("Force quit");

  let mut i = 61;
  for c in "Hello world!".chars() {
    i += 1;
    os.screen.cells[i] = c as u8;
  }
  for i in 0..40 {
    if os.inputs[i] > 0 {
      os.screen.cells[i] = 33;
    } else {
      os.screen.cells[i] = 95;
    }
  }
  //let s = std::ffi::CString::new(format!("Hello 😃")).unwrap();
  //let slice = s.as_bytes();
  //prn(slice.as_ptr(), slice.len());
}
