pub struct AppState {
    pub time: f64,
    pub program: ProgramState,
    pub scale: u8,
    pub ui_visible: bool,
    pub clear_required: bool,
    pub selected_element: UiSelectedElement,
}

pub enum ProgramState {
    None,
    Perlin(crate::perlin::PerlinState),
    Matrix(crate::matrix::MatrixState),
    Roguelike(crate::roguelike::RoguelikeState),
}
impl ProgramState {
    pub fn display_name(&self) -> &str {
        match self {
            ProgramState::None => "None",
            ProgramState::Perlin(_) => "Perlin noise",
            ProgramState::Matrix(_) => "Matrix",
            ProgramState::Roguelike(_) => "Roguelike",
        }
    }
}

pub enum UiSelectedElement {
    ProgramSelector,
    ScaleSelector,
}

pub enum UiAction {
    Left,
    Right,
    Accept,
    Up,
    Down,
    HideUI,
}

impl AppState {
    fn start_matrix(&mut self) {
        use crate::matrix::MatrixState;
        self.program = ProgramState::Matrix(MatrixState::new());
        self.clear_required = true;
    }
    fn start_perlin(&mut self) {
        use crate::perlin::PerlinState;
        self.program = ProgramState::Perlin(PerlinState::new());
        self.clear_required = true;
    }
    fn start_roguelike(&mut self) {
        use crate::roguelike::RoguelikeState;
        self.program = ProgramState::Roguelike(RoguelikeState::new());
        self.clear_required = true;
    }
    fn next_program(&mut self) {
        match self.program {
            ProgramState::None => self.start_matrix(),
            ProgramState::Roguelike(_) => self.start_matrix(),
            ProgramState::Matrix(_) => self.start_perlin(),
            ProgramState::Perlin(_) => self.start_roguelike(),
        }
    }

    fn prev_program(&mut self) {
        match self.program {
            ProgramState::None => self.start_perlin(),
            ProgramState::Roguelike(_) => self.start_perlin(),
            ProgramState::Matrix(_) => self.start_roguelike(),
            ProgramState::Perlin(_) => self.start_matrix(),
        }
    }
    fn adjust_scale(&mut self, by: i32) {
        let new_count = (self.scale as i32) + by;
        if new_count < 1 || new_count > 4 {
            return;
        }
        self.clear_required = true;
        self.scale = new_count as u8;
    }

    fn left(&mut self) {
        match self.selected_element {
            UiSelectedElement::ProgramSelector => self.prev_program(),
            UiSelectedElement::ScaleSelector => self.adjust_scale(-1),
        }
    }
    fn right(&mut self) {
        match self.selected_element {
            UiSelectedElement::ProgramSelector => self.next_program(),
            UiSelectedElement::ScaleSelector => self.adjust_scale(1),
        }
    }
    fn accept(&mut self) {
        match self.selected_element {
            UiSelectedElement::ProgramSelector => self.next_program(),
            _ => (),
        }
    }
    fn up(&mut self) {
        match self.selected_element {
            UiSelectedElement::ProgramSelector => (),
            UiSelectedElement::ScaleSelector => {
                self.selected_element = UiSelectedElement::ProgramSelector
            }
        };
    }
    fn down(&mut self) {
        match self.selected_element {
            UiSelectedElement::ProgramSelector => {
                self.selected_element = UiSelectedElement::ScaleSelector
            }
            UiSelectedElement::ScaleSelector => ()
        };
    }
    pub fn dispatch(&mut self, action: UiAction) {
        match action {
            UiAction::Left => self.left(),
            UiAction::Right => self.right(),
            UiAction::Accept => self.accept(),
            UiAction::Up => self.up(),
            UiAction::Down => self.down(),
            UiAction::HideUI => self.ui_visible = !self.ui_visible,
        }
    }
}
