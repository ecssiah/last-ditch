pub mod id;
pub mod kind;

pub use id::ID;
pub use kind::Kind;

use crate::simulation::{
    population::decision::{Decision, Goal, Step},
    time::Tick,
    world::World,
    FIXED_DT,
};
use glam::Vec3;
use rand::{Rng, SeedableRng};

#[derive(Clone)]
pub struct Agent {
    pub id: ID,
    pub tick: Tick,
    pub name: String,
    pub position: Vec3,
    pub kind: Kind,
    pub decision: Decision,
    pub plan: Vec<Step>,
    pub step_index: usize,
    pub target: Vec3,
    pub speed: f32,
    pub height: f32,
}

impl Agent {
    pub fn new(agent_id: ID) -> Agent {
        let agent = Self {
            id: agent_id,
            tick: Tick::ZERO,
            name: "TEST AGENT NAME".to_string(),
            position: Vec3::ZERO,
            kind: Kind::Lion,
            decision: Decision::new(),
            plan: Vec::new(),
            step_index: 0,
            target: Vec3::ZERO,
            speed: 1.0,
            height: 1.5,
        };

        agent
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = Vec3::new(x, y, z);
    }

    pub fn tick(&mut self, world: &World) {
        if self.plan.is_empty() {
            let mut rng = rand_pcg::Pcg32::from_entropy();

            let flip = rng.gen_bool(0.5);

            if flip {
                self.plan = self.decision.plan(&Goal::Idle, self, world);
            } else {
                self.plan = self.decision.plan(&Goal::Wander, self, world);
                
                log::info!("{:?}", self.plan);
            }
        }

        if self.step_index < self.plan.len() {
            if let Some(step) = self.plan.get(self.step_index) {
                match step {
                    Step::Move(target_position) => {
                        let path = target_position.as_vec3() - self.position;

                        if path.length_squared() > 1e-2 {
                            self.position += self.speed * FIXED_DT.as_secs_f32() * path.normalize();
                        } else {
                            self.step_index += 1;
                        }
                    }
                    Step::Wait => {
                        self.step_index += 1;
                    }
                }
            }
        } else {
            self.step_index = 0;
            self.plan.clear();
        }
    }
}
