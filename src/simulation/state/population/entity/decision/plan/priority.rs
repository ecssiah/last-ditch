#[derive(Hash, PartialEq, Eq)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl Priority {
    pub const ALL_ARRAY: [Priority; 3] = [Priority::High, Priority::Medium, Priority::Low];
}
