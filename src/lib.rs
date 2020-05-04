const COLS: u8 = 90;
const ROWS: u8 = 40;

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
  cols: COLS,
  rows: ROWS,
};

#[no_mangle]
static mut SCREEN: [u8; 4800] = [0; 4800];

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
    -10..=10 => 0,
    -30..=30 => 249,
    -70..=70 => 176,
    -90..=90 => 177,
    -100..=100 => 178,
    -120..=120 => 219,
    _ => 2,
  }
}

#[no_mangle]
pub extern "C" fn frame() {
  std::panic::set_hook(Box::new(|p| {
    log(format!("Panic at {:?}", p));
  }));

  unsafe {
    FRAME_NUMBER += 1;
    for y in 0..ROWS {
      for x in 0..COLS {
        let i: usize = (y as usize) * (COLS as usize) + (x as usize);
        let dx = (x as f32) - (COLS as f32) / 2.0;
        let dy = (y as f32) - (ROWS as f32) / 2.0;

        //let dist = (dx.powi(2) + dy.powi(2)).sqrt();
        //let t = ((FRAME_NUMBER as f32) + (dist / 10.0)) / 120.0;
        let t = (FRAME_NUMBER as f32) / 60.0;

        let fac_y = t.sin() * 3.0;

        SCREEN[i] = float_to_blocky_char(
          ((x as f32 + t * 60.0) / 12.0).sin() - ((y as f32) / (6.0 + fac_y) - t * 4.0).cos(),
        );
      }
    }

    SCREEN[2] = 255;
    SCREEN[3] = 66;
    //let s = std::ffi::CString::new(format!("Hello 😃")).unwrap();
    //let slice = s.as_bytes();
    //prn(slice.as_ptr(), slice.len());
  }
}
