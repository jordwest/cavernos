extern "C" {
    fn prn(v: *const u8, len: usize);
}

// Remove this if you don't need heap memory allocation
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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

pub fn log(s: &str) {
    let s = std::ffi::CString::new(s).unwrap();
    let slice = s.as_bytes();
    unsafe {
        prn(slice.as_ptr(), slice.len());
    }
}

/// Set up a panic handler so that if our program panics, the error message is
/// forwarded to the runtime console
pub fn init_panic_hook() {
  std::panic::set_hook(Box::new(|p| {
    log(&format!("Panic at {:?}", p));
  }));
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

impl CavernOS {
    pub fn get_input_latch(&mut self, index: usize) -> bool {
        let value = self.inputs[index];
        self.inputs[index] = 0;
        value > 0
    }
    pub fn clear(&mut self, bg_color: u8) {
        let last_index = self.config.cols as usize * self.config.rows as usize;
        let mut i = 0;
        self.screen.update = true;
        self.bg_color.update = true;
        self.fg_color.update = true;
        while i < last_index {
            self.screen.cells[i] = 0;
            self.fg_color.cells[i] = 0;
            self.bg_color.cells[i] = bg_color;
            i += 1;
        }
    }
    pub fn cell_at(&self, x: u8, y: u8) -> usize {
        (y as usize) * (self.config.cols as usize) + (x as usize)
    }
    pub fn set_char(&mut self, x: u8, y: u8, c: u8) {
        let cell = self.cell_at(x, y);
        self.screen.cells[cell] = c;
        self.screen.update = true;
    }
    pub fn set_bg_color(&mut self, x: u8, y: u8, color: u8) {
        let cell = self.cell_at(x, y);
        self.bg_color.cells[cell] = color;
        self.bg_color.update = true;
    }
    pub fn set_fg_color(&mut self, x: u8, y: u8, color: u8) {
        let cell = self.cell_at(x, y);
        self.fg_color.cells[cell] = color;
        self.fg_color.update = true;
    }
    pub fn draw_string(&mut self, x: u8, y: u8, text: &str, bg_color: u8, fg_color: u8) {
      let mut x = x;
      for c in text.chars() {
        if c.is_ascii() {
          self.set_char(x, y, c as u8);
          self.set_bg_color(x, y, bg_color);
          self.set_fg_color(x, y, fg_color);
          x += 1;
        }
      }
    }
}
