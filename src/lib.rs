mod cavernos;
mod perlin;
mod state;

use state::AppState;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
static mut STATE: AppState = AppState { time: 0.0 };

#[no_mangle]
extern "C" fn init(_os: &mut cavernos::CavernOS) -> &mut AppState {
  std::panic::set_hook(Box::new(|p| {
    cavernos::log(format!("Panic at {:?}", p));
  }));

  // This is the only unsafe we'll need. We're passing a pointer to our
  // app's state
  return unsafe { &mut STATE };
}

#[no_mangle]
extern "C" fn frame(os: &mut cavernos::CavernOS, app_state: &mut AppState, dt: f64) {
  app_state.time += dt;
  perlin::draw(os, app_state);
}

