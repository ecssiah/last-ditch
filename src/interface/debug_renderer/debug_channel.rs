#[repr(u8)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum DebugChannel {
    SectorBorders,
    Custom,
}

impl DebugChannel {
    pub const ALL: [Self; 2] = [Self::SectorBorders, Self::Custom];
}
