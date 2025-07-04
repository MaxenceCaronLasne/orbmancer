#[derive(Clone, Copy, Debug)]
pub enum Effect {
    Identity,
    AddMult(i32),
    AddBase(i32),
}

impl Effect {
    pub fn apply(self, base: i32, mult: i32) -> (i32, i32) {
        match self {
            Effect::Identity => (base, mult),
            Effect::AddMult(m) => (base, mult + m),
            Effect::AddBase(b) => (base + b, mult),
        }
    }
}
