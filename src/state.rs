pub struct AppState {
  pub time: f64,
  pub program: ProgramState,
}

pub enum ProgramState {
  None,
  Perlin(crate::perlin::PerlinState),
  Matrix(crate::matrix::MatrixState),
}
