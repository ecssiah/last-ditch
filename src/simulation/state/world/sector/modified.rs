pub struct Modified {
    pub cell: bool,
    pub edge: bool,
}

impl Modified {
    pub fn new() -> Self {
        Self {
            cell: false,
            edge: false,
        }
    }
}
