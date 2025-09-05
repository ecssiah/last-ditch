use crate::interface::debug::DebugVisibility;

#[derive(Copy, Clone)]
pub enum DebugChannel {
    Channel1,
    Channel2,
    Channel3,
    Channel4,
    ChunkBorders,
}

impl DebugChannel {
    pub const ALL: [DebugChannel; 5] = [
        DebugChannel::Channel1,
        DebugChannel::Channel2,
        DebugChannel::Channel3,
        DebugChannel::Channel4,
        DebugChannel::ChunkBorders,
    ];

    pub fn mask(self) -> DebugVisibility {
        match self {
            DebugChannel::Channel1 => DebugVisibility::CHANNEL1,
            DebugChannel::Channel2 => DebugVisibility::CHANNEL2,
            DebugChannel::Channel3 => DebugVisibility::CHANNEL3,
            DebugChannel::Channel4 => DebugVisibility::CHANNEL4,
            DebugChannel::ChunkBorders => DebugVisibility::CHUNK_BORDERS,
        }
    }

    pub fn index(self) -> usize {
        self as usize
    }
}
