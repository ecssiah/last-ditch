use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct State {
    pub time: f64,
}

pub type SharedState = Arc<RwLock<State>>;
