use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_OBSERVATION_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ObservationID(pub usize);

impl ObservationID {
    pub fn allocate() -> ObservationID {
        ObservationID(NEXT_OBSERVATION_ID.fetch_add(1, Ordering::Relaxed))
    }
}

impl From<ObservationID> for usize {
    fn from(observation_id: ObservationID) -> Self {
        observation_id.0
    }
}
