pub enum TASToken {
    TASInput(TASInputLine),
}

pub struct TASInputLine {
    pub frame_count: usize,
    pub input: TASInput,
}

pub struct TASInput {
    pub jump: Jump,
    pub dash: Dash,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Jump {
    J,
    K,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dash {
    X,
    C,
    Z,
    V,
}

impl Dash {
    pub fn is_crouch_dash(self) -> bool {
        match self {
            Dash::Z | Dash::V => true,
            Dash::X | Dash::C => false,
        }
    }
}
