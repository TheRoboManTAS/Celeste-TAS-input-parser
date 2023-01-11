#[derive(Default, Debug, PartialEq, Clone)]
pub struct TASInput {
    pub jump: Option<Jump>,
    pub dash: Option<Dash>,
    pub grab: bool,
    pub directional: DirectionalInput,
    pub dash_only: DpadInput,
    pub confirm: bool,
    pub pause: bool,
    pub quick_restart: bool,
    pub journal: bool,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DirectionalInput {
    Dpad(DpadInput),
    Analog(f64, Option<f64>),
}

impl Default for DirectionalInput {
    fn default() -> Self {
        Self::Dpad(DpadInput::default())
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DpadInput {
    pub horizontal: Horizontal,
    pub vertical: Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub enum Horizontal {
    Left,
    Right,
    #[default]
    None,
}

impl Horizontal {
    pub fn inverted(self) -> Horizontal {
        match self {
            Horizontal::Left => Horizontal::Right,
            Horizontal::Right => Horizontal::Left,
            Horizontal::None => Horizontal::None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub enum Vertical {
    Up,
    Down,
    #[default]
    None,
}

impl Vertical {
    pub fn inverted(self) -> Vertical {
        match self {
            Vertical::Up => Vertical::Down,
            Vertical::Down => Vertical::Up,
            Vertical::None => Vertical::None,
        }
    }
}

impl TASInput {
    pub fn confirms(&self) -> bool {
        self.confirm || matches!(self.jump, Some(Jump::J))
    }

    pub fn cancels(&self) -> bool {
        matches!(self.dash, Some(Dash::X | Dash::C))
    }

    pub fn talks(&self) -> bool {
        matches!(self.dash, Some(Dash::X)) || self.journal
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Jump {
    J,
    K,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
