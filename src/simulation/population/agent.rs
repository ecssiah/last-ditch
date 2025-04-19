pub mod id;

pub use id::ID;

use crate::simulation::time::Tick;
use glam::Vec3;

#[derive(Clone)]
pub struct Agent {
    pub id: ID,
    pub tick: Tick,
    pub name: &'static str,
    pub position: Vec3,
    pub target: Vec3,
}

impl Agent {
    pub fn new(agent_id: ID) -> Agent {
        let agent = Self {
            id: agent_id,
            tick: Tick::ZERO,
            name: "",
            position: Vec3::ZERO,
            target: Vec3::ZERO,
        };

        agent
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = Vec3::new(x, y, z);
    }
}
