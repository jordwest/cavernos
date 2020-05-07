mod cavernos;
mod matrix;
mod perlin;
mod state;

use state::{AppState, ProgramState};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
static mut STATE: AppState = AppState {
  time: 0.0,
  program: ProgramState::None,
};

#[no_mangle]
extern "C" fn init(_os: &mut cavernos::CavernOS) -> &mut AppState {
  std::panic::set_hook(Box::new(|p| {
    cavernos::log(format!("Panic at {:?}", p));
  }));

  // This is the only unsafe we'll need. We're getting a pointer to our
  // app's state once, and then passing it back to the host.
  let mut state = unsafe { &mut STATE };

  //state.program = ProgramState::Perlin(perlin::PerlinState::new());
  state.program = ProgramState::Matrix(matrix::MatrixState::new());

  return state;
}

#[no_mangle]
extern "C" fn frame(os: &mut cavernos::CavernOS, app_state: &mut AppState, dt: f64) {
  app_state.time += dt;
  match &mut app_state.program {
    ProgramState::None => (),
    ProgramState::Perlin(ref mut p) => perlin::frame(os, p, dt),
    ProgramState::Matrix(ref mut m) => matrix::frame(os, m, dt),
  }
}

