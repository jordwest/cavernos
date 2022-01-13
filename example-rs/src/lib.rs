mod cavernos;
mod geometry;
mod matrix;
mod perlin;
mod roguelike;
mod state;
mod ui;

use geometry::rect::Rect;
use state::{AppState, ProgramState, UiAction, UiSelectedElement};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
static mut STATE: AppState = AppState {
    time: 0.0,
    scale: 2u8,
    program: ProgramState::None,
    ui_visible: true,
    clear_required: false,
    selected_element: UiSelectedElement::ProgramSelector,
};

#[no_mangle]
extern "C" fn init(os: &mut cavernos::CavernOS) -> &mut AppState {
    std::panic::set_hook(Box::new(|p| {
        cavernos::log(format!("Panic at {:?}", p));
    }));
    os.config.cols = 90;
    os.config.rows = 24;

    // This is the only unsafe we'll need. We're getting a pointer to our
    // app's state once, and then passing it back to the host.
    let mut state = unsafe { &mut STATE };

    state.program = ProgramState::Matrix(matrix::MatrixState::new());

    return state;
}

// Inputs
const MOVE_UP: usize = 3;
const MOVE_DOWN: usize = 2;
const MOVE_LEFT: usize = 1;
const MOVE_RIGHT: usize = 4;
const KEY_H: usize = 104;

const UI_WIDTH: u8 = 28;

#[no_mangle]
extern "C" fn frame(os: &mut cavernos::CavernOS, app_state: &mut AppState, dt: f64) {
    if os.get_input_latch(MOVE_UP) {
        app_state.dispatch(UiAction::Up);
    } else if os.get_input_latch(MOVE_DOWN) {
        app_state.dispatch(UiAction::Down);
    } else if os.get_input_latch(MOVE_LEFT) {
        app_state.dispatch(UiAction::Left);
    } else if os.get_input_latch(MOVE_RIGHT) {
        app_state.dispatch(UiAction::Right);
    } else if os.get_input_latch(KEY_H) {
      app_state.dispatch(UiAction::HideUI);
    }

    if app_state.clear_required {
        app_state.clear_required = false;
        os.clear();
    }
  
    app_state.time += dt;
    match &mut app_state.program {
        ProgramState::None => (),
        ProgramState::Perlin(ref mut p) => perlin::frame(os, p, dt),
        ProgramState::Matrix(ref mut m) => matrix::frame(os, m, dt),
        ProgramState::Roguelike(ref mut s) => roguelike::frame(os, s, dt),
    }

    os.config.rows = os.config.max_rows / app_state.scale;
    os.config.cols = (os.config.max_cols / app_state.scale);
  
    // Ensure we have an even number of columns (so that square characters wrap correctly - they use two chars)
    os.config.cols = os.config.cols / 2 * 2;

    if app_state.ui_visible {
        let ui_rect = Rect {
            top: 1,
            left: (os.config.cols - UI_WIDTH - 2) as i32,
            right: (os.config.cols - 2) as i32,
            bottom: (os.config.rows - 2) as i32,
        };
        ui::frame(os, app_state, ui_rect);
    }
}
