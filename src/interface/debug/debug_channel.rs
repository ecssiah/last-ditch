use crate::interface::debug::DebugVisibility;

#[derive(Copy, Clone)]
pub enum DebugChannel {
    Channel1,
    Channel2,
    Channel3,
    Channel4,
    SectorBorders,
}

impl DebugChannel {
    pub const ALL: [DebugChannel; 5] = [
        DebugChannel::Channel1,
        DebugChannel::Channel2,
        DebugChannel::Channel3,
        DebugChannel::Channel4,
        DebugChannel::SectorBorders,
    ];

    pub fn mask(debug_channel: DebugChannel) -> DebugVisibility {
        match debug_channel {
            DebugChannel::Channel1 => DebugVisibility::CHANNEL1,
            DebugChannel::Channel2 => DebugVisibility::CHANNEL2,
            DebugChannel::Channel3 => DebugVisibility::CHANNEL3,
            DebugChannel::Channel4 => DebugVisibility::CHANNEL4,
            DebugChannel::SectorBorders => DebugVisibility::SECTOR_BORDERS,
        }
    }

    pub fn index(debug_channel: DebugChannel) -> usize {
        debug_channel as usize
    }
}
