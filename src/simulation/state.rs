use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct State {
    pub user: Arc<RwLock<User>>,
    pub world: Arc<RwLock<World>>,
}

#[derive(Debug, Clone)]
pub struct User {
    
}

#[derive(Debug, Clone)]
pub struct World {
    pub active: bool,
    pub seed: u64,
    pub time: f64,
}
