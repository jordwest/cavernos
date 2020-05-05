use noise::{NoiseFn, Seedable};

extern "C" {
  fn prn(v: *const u8, len: usize);
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[repr(C)]
#[no_mangle]
#[derive(Debug)]
struct Config {
  cols: u8,
  rows: u8,
}

#[no_mangle]
static mut CONFIG: Config = Config {
  cols: 120,
  rows: 40,
};

#[no_mangle]
static mut SCREEN: [u8; 4800] = [0; 4800];

#[no_mangle]
static mut INPUTS: [u8; 256] = [0; 256];

static mut FRAME_NUMBER: usize = 0;

fn log(s: String) {
  let s = std::ffi::CString::new(s).unwrap();
  let slice = s.as_bytes();
  unsafe {
    prn(slice.as_ptr(), slice.len());
  }
}

fn float_to_blocky_char(val: f32) -> u8 {
  let i = (val * 100.0) as i32;
  match i {
    -10..=10 => 1,
    -30..=30 => 2,
    -70..=70 => 3,
    -90..=90 => 14,
    -100..=100 => 177,
    -120..=120 => 178,
    _ => 2,
  }
}

#[no_mangle]
pub extern "C" fn frame() {
  std::panic::set_hook(Box::new(|p| {
    log(format!("Panic at {:?}", p));
  }));

  unsafe {
    if INPUTS[2] > 0 {
      FRAME_NUMBER -= 1;
    } else if INPUTS[3] > 0 {
      FRAME_NUMBER += 2;
    } else if INPUTS[5] > 0 {
      // Do nothing
    } else {
      FRAME_NUMBER += 1;
    }
    let t = (FRAME_NUMBER as f32) / 60.0;

    let cols = CONFIG.cols;
    let rows = CONFIG.rows;
    let n = noise::Perlin::new();
    let n = n.set_seed(18);
    let v = n.get([16.3, 32.0, 8.0]);
    //log(format!("{:?}", v));

    for y in 0..rows {
      for x in 0..cols {
        let v = n.get([
          x as f64 / cols as f64,
          y as f64 / rows as f64,
          FRAME_NUMBER as f64 / 64.0,
        ]);
        //let v = n.get([2.1, 4.6, 8.0]);
        //log(format!("{:?}", v));

        let i: usize = (y as usize) * (cols as usize) + (x as usize);
        let dx = (x as f32) - (cols as f32) / 2.0;
        let dy = (y as f32) - (rows as f32) / 2.0;

        //let dist = (dx.powi(2) + dy.powi(2)).sqrt();
        //let t = ((FRAME_NUMBER as f32) + (dist / 10.0)) / 120.0;

        let fac_y = t.sin() * 3.0;

        if i % 2 == 0 {
          SCREEN[i] = 255;
          //SCREEN[i + 1] = float_to_blocky_char(
          //  ((x as f32 + t * 60.0) / 12.0).sin() - ((y as f32) / (6.0 + fac_y) - t * 4.0).cos(),
          //);
          SCREEN[i + 1] = float_to_blocky_char(v as f32);
        }
      }
    }

    //panic!("Force quit");

    let mut i = 61;
    for c in "Hello world!".chars() {
      i += 1;
      SCREEN[i] = c as u8;
    }
    for i in 0..40 {
      if INPUTS[i] > 0 {
        SCREEN[i] = 33;
      } else {
        SCREEN[i] = 95;
      }
    }
    //let s = std::ffi::CString::new(format!("Hello 😃")).unwrap();
    //let slice = s.as_bytes();
    //prn(slice.as_ptr(), slice.len());
  }
}
