const COLS: u8 = 80;
const ROWS: u8 = 60;

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
static mut SCREEN: [u8; 4800] = [63; 4800];

static mut FRAME_NUMBER: usize = 0;

#[no_mangle]
pub extern "C" fn frame() {
  unsafe {
    FRAME_NUMBER += 1;
    for i in 0..4800 {
      SCREEN[i] = ((i + FRAME_NUMBER) % 255) as u8;
    }
    //let s = std::ffi::CString::new(format!("Hello 😃")).unwrap();
    //let slice = s.as_bytes();
    //prn(slice.as_ptr(), slice.len());
  }
}
