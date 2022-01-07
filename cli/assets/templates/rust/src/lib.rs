mod cavernos;

// This gives us a place to store all the state we need between frames
struct AppState {
  time: f64,
}

#[no_mangle]
static mut STATE: AppState = AppState {
  time: 0.0,
};

#[no_mangle]
extern "C" fn init(os: &mut cavernos::CavernOS) -> &mut AppState {
  cavernos::init_panic_hook();
  os.config.cols = 80;
  os.config.rows = 24;

  // Print to runtime console
  cavernos::log("Hello world!");

  // This is the only unsafe we'll need. We're getting a pointer to our
  // app's state once, and then passing it back to the host.
  let state = unsafe { &mut STATE };
  
  return state;
}

const SMILEY_OUTLINE: u8 = 1;
const SMILEY_FILLED: u8 = 2;

const BLACK: u8 = 0;
const WHITE: u8 = 1;

#[no_mangle]
extern "C" fn frame(os: &mut cavernos::CavernOS, app_state: &mut AppState, dt: f64) {
  app_state.time += dt;

  os.clear(16);

  let mut x: u8 = 28;
  let y: u8 = 12;

  // Smiley 1
  os.set_char(x, y, SMILEY_OUTLINE);
  os.set_fg_color(x, y, WHITE);

  x += 2;
  let message = format!("Running for {:.2}s", app_state.time);
  os.draw_string(x, y, &message, BLACK, WHITE);
  x += message.len() as u8;

  // Smiley 2
  x += 1;
  os.set_char(x, y, SMILEY_FILLED);
  os.set_fg_color(x, y, WHITE);
}

