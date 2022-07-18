#[derive(Copy, Clone)]
pub enum Shape {
    S0 = 0,
}

impl Shape {
    pub fn from_id(id: u8) -> Option<Self> {
        let shape = match id {
            0 => Self::S0,
            _ => return None,
        };

        Some(shape)
    }
}
