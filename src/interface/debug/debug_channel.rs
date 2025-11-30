#[repr(u8)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum DebugChannel {
    SectorBorders,
    Custom,
}

impl DebugChannel {
    pub const ALL: [DebugChannel; 2] = [DebugChannel::SectorBorders, DebugChannel::Custom];
}
