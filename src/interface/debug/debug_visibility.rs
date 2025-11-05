bitflags::bitflags! {
    #[derive(Default)]
    pub struct DebugVisibility: u32 {
        const CHANNEL1              = 1 << 0;
        const CHANNEL2              = 1 << 1;
        const CHANNEL3              = 1 << 2;
        const CHANNEL4              = 1 << 3;
        const SECTOR_BORDERS         = 1 << 4;
    }
}
