bitflags::bitflags! {
    #[derive(Default)]
    pub struct DebugVisibility: u32 {
        const CHANNEL1              = 1 << 0;
        const CHANNEL2              = 2 << 0;
        const CHANNEL3              = 3 << 0;
        const CHANNEL4              = 4 << 0;
        const CHUNK_BORDERS         = 5 << 0;
    }
}
