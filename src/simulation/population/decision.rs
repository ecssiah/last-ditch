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

        if let Some(grid_position) = world.grid.world_to_grid(agent.position) {
            for _ in 0..10 {
                if let Some(next_grid_position) = Self::find_target(&grid_position, agent, world) {
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

    fn find_target(grid_position: &IVec3, agent: &Agent, world: &World) -> Option<IVec3> {
        let mut rng = rand::thread_rng();

        let direction_index = rng.gen_range(0..4);
        let direction = grid::Direction::cardinal_list()[direction_index];

        let dy = rng.gen_range(-1..=1);
        let offset = direction.offset() + IVec3::new(0, dy, 0);
        let target_position = grid_position + offset;

        let required_clearance = agent.height.ceil() as i32;

        if world.has_clearance(target_position, required_clearance) {
            Some(target_position)
        } else {
            None
        }
    }
}
