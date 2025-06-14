pub mod id;
pub mod kind;

pub use id::ID;
pub use kind::Kind;

use crate::simulation::{
    population::decision::{Decision, Goal, Step},
    time::Tick,
    world::World,
    SIMULATION_TICK_DURATION,
};
use glam::Vec3;
use rand::Rng;

#[derive(Clone)]
pub struct Agent {
    pub id: ID,
    pub tick: Tick,
    pub world_position: Vec3,
    pub target_world_position: Vec3,
    pub kind: Kind,
    pub decision: Decision,
    pub plan: Vec<Step>,
    pub step_index: usize,
    pub speed: f32,
    pub height: f32,
}

impl Agent {
    pub fn new(agent_id: ID) -> Self {
        Self {
            id: agent_id,
            tick: Tick::ZERO,
            world_position: Vec3::ZERO,
            target_world_position: Vec3::ZERO,
            kind: Kind::Lion,
            decision: Decision::new(),
            plan: Vec::new(),
            step_index: 0,
            speed: 1.0,
            height: 1.5,
        }
    }

    pub fn tick(&mut self, world: &World) {
        if self.plan.is_empty() {
            let mut rng = rand::thread_rng();

            let flip = rng.gen_bool(0.5);

            if flip {
                self.plan = self.decision.plan(&Goal::Idle, self, world);
            } else {
                self.plan = self.decision.plan(&Goal::Wander, self, world);
            }
        }

        if self.step_index < self.plan.len() {
            if let Some(step) = self.plan.get(self.step_index) {
                match step {
                    Step::Move(target_position) => {
                        let path = target_position.as_vec3() - self.world_position;

                        if path.length_squared() > 1e-2 {
                            self.world_position += self.speed
                                * SIMULATION_TICK_DURATION.as_secs_f32()
                                * path.normalize();
                        } else {
                            self.world_position = target_position.as_vec3();
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
