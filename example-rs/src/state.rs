pub struct AppState {
    pub time: f64,
    pub program: ProgramState,
    pub rows: u8,
    pub cols: u8,
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
    RowSelector,
    ColumnSelector,
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
    fn adjust_rows(&mut self, by: i32) {
        let new_count = (self.rows as i32) + (2 * by);
        if new_count < 20 || new_count > 255 {
            return;
        }
        self.clear_required = true;
        self.rows = new_count as u8;
    }
    fn adjust_columns(&mut self, by: i32) {
        let new_count = (self.cols as i32) + (2 * by);
        if new_count < 60 || new_count > 255 {
            return;
        }
        self.clear_required = true;
        self.cols = new_count as u8;
    }

    fn left(&mut self) {
        match self.selected_element {
            UiSelectedElement::ProgramSelector => self.prev_program(),
            UiSelectedElement::RowSelector => self.adjust_rows(-1),
            UiSelectedElement::ColumnSelector => self.adjust_columns(-1),
        }
    }
    fn right(&mut self) {
        match self.selected_element {
            UiSelectedElement::ProgramSelector => self.next_program(),
            UiSelectedElement::RowSelector => self.adjust_rows(1),
            UiSelectedElement::ColumnSelector => self.adjust_columns(1),
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
            UiSelectedElement::RowSelector => {
                self.selected_element = UiSelectedElement::ProgramSelector
            }
            UiSelectedElement::ColumnSelector => {
                self.selected_element = UiSelectedElement::RowSelector
            }
        };
    }
    fn down(&mut self) {
        match self.selected_element {
            UiSelectedElement::ProgramSelector => {
                self.selected_element = UiSelectedElement::RowSelector
            }
            UiSelectedElement::RowSelector => {
                self.selected_element = UiSelectedElement::ColumnSelector
            }
            UiSelectedElement::ColumnSelector => (),
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
