mod tas_input;

pub use tas_input::TASInput;

pub enum TASToken {
    TASInput(TASInputLine),
}

pub struct TASInputLine {
    pub frame_count: usize,
    pub input: TASInput,
}
