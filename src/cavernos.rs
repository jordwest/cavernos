extern "C" {
  fn prn(v: *const u8, len: usize);
}

#[repr(C)]
pub struct ScreenData {
  pub update: bool,
  pub cells: [u8; 65535],
}
impl ScreenData {
  pub const fn new() -> ScreenData {
    ScreenData {
      update: false,
      cells: [0; 65535],
    }
  }
}

#[repr(C)]
pub struct Config {
  pub cols: u8,
  pub rows: u8,
  __reserved: [u8; 254],
}

pub fn log(s: String) {
  let s = std::ffi::CString::new(s).unwrap();
  let slice = s.as_bytes();
  unsafe {
    prn(slice.as_ptr(), slice.len());
  }
}

#[no_mangle]
static mut OS: CavernOS = CavernOS {
  config: Config {
    cols: 120,
    rows: 40,
    __reserved: [0; 254],
  },
  inputs: [0; 256],
  __reserved: [0; 2560],
  screen: ScreenData::new(),
  bg_color: ScreenData::new(),
  fg_color: ScreenData::new(),
};

#[repr(C)]
pub struct CavernOS {
  pub config: Config,
  pub inputs: [u8; 256],
  __reserved: [u8; 2560],

  pub screen: ScreenData,
  pub bg_color: ScreenData,
  pub fg_color: ScreenData,
}

