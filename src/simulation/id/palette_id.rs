use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_PALETTE_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PaletteID(pub usize);

impl PaletteID {
    pub fn allocate() -> PaletteID {
        PaletteID(NEXT_PALETTE_ID.fetch_add(1, Ordering::Relaxed))
    }
}

impl From<PaletteID> for usize {
    fn from(agent_id: PaletteID) -> Self {
        agent_id.0
    }
}
