#[derive(Clone, Hash, PartialEq, Eq)]
pub struct DoorData {
    pub is_open: bool,
    pub is_locked: bool,
}

impl DoorData {
    pub fn new() -> Self {
        Self {
            is_open: false,
            is_locked: false,
        }
    }
}
