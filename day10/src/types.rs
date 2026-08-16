#[derive(Clone, Debug)]
pub enum Light {
    On,
    Off,
}

pub type Lights = Vec<Light>;
pub type Wiring = Vec<usize>;
pub type Buttons = Vec<Wiring>;
pub type Joltage = Vec<usize>;

#[derive(Clone, Debug)]
pub struct Machine {
    pub(crate) lights: Lights,
    pub(crate) buttons: Buttons,
    pub(crate) joltage: Joltage,
}

impl Machine {
    pub(crate) fn new(lights: Lights, buttons: Buttons, joltage: Joltage) -> Self {
        Self {
            lights,
            buttons,
            joltage,
        }
    }
}
