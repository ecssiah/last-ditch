pub mod goal;
pub mod step;

pub use goal::Goal;
pub use step::Step;

use crate::simulation::{
    population::Agent,
    world::{grid, World},
};
use glam::IVec3;
use rand::Rng;

#[derive(Clone)]
pub struct Decision {
    pub goal: Option<Goal>,
    pub plan: Vec<Step>,
    pub step: Option<Step>,
}

impl Decision {
    pub fn new() -> Decision {
        let decision = Decision {
            goal: None,
            plan: Vec::new(),
            step: None,
        };

        decision
    }

    pub fn plan(&self, goal: &Goal, agent: &Agent, world: &World) -> Vec<Step> {
        match goal {
            Goal::Idle => self.plan_idle(),
            Goal::Wander => self.plan_wander(agent, world),
            Goal::Seek(target_position) => self.plan_seek(&target_position, agent, world),
        }
    }

    fn plan_idle(&self) -> Vec<Step> {
        let mut rng = rand::thread_rng();

        let mut plan = Vec::new();
        let wait_steps = rng.gen_range(28..48);

        for _ in 0..wait_steps {
            plan.push(Step::Wait);
        }

        plan
    }

    fn plan_wander(&self, agent: &Agent, world: &World) -> Vec<Step> {
        let mut plan = Vec::new();

        if let Some(grid_position) = grid::world_to_grid(agent.position) {
            for _ in 0..10 {
                if let Some(next_grid_position) = Self::find_target(&grid_position, world) {
                    let step = Step::Move(next_grid_position);

                    plan.push(step);
                }
            }
        }

        plan
    }

    fn plan_seek(&self, _target: &IVec3, _agent: &Agent, _world: &World) -> Vec<Step> {
        Vec::new()
    }

    fn find_target(grid_position: &IVec3, world: &World) -> Option<IVec3> {
        let mut rng = rand::thread_rng();

        let direction_index = rng.gen_range(0..4);
        let direction = grid::Direction::cardinal()[direction_index];

        let dy = rng.gen_range(-1..=1);

        let delta = match direction {
            grid::Direction::XpYoZo => IVec3::new(1, dy, 0),
            grid::Direction::XnYoZo => IVec3::new(-1, dy, 0),
            grid::Direction::XoYoZp => IVec3::new(0, dy, 1),
            grid::Direction::XoYoZn => IVec3::new(0, dy, -1),
            _ => IVec3::ZERO,
        };

        let target_position = grid_position + delta;

        if world.is_clear(target_position, 3) {
            Some(target_position)
        } else {
            None
        }
    }
}
