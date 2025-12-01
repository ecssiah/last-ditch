pub mod state;
pub mod tree;

pub use state::State;
pub use tree::Tree;

use crate::simulation::state::{
    navigation::Navigation,
    population::{agent::Agent, behavior},
    world::grid,
};
use ultraviolet::IVec3;

pub struct Behavior {
    pub tree: behavior::Tree,
    pub state: behavior::State,
}

impl Behavior {
    pub fn new() -> Self {
        let tree = behavior::Tree::new();
        let state = behavior::State::Idle;

        Self { tree, state }
    }

    pub fn tick(navigation: &mut Navigation, agent: &mut Agent) {
        match &agent.behavior.state {
            behavior::State::Idle => (),
            behavior::State::Navigating {
                grid_position,
                path_request_id,
            } => Self::handle_navigating_state(
                grid::world_position_to_grid_position(agent.spatial.world_position),
                *grid_position,
                *path_request_id,
                navigation,
                &mut agent.behavior,
            ),
            behavior::State::Moving { path_vec } => {
                Self::handle_moving_state(path_vec.clone(), &mut agent.behavior)
            }
        }
    }

    fn handle_navigating_state(
        start_grid_position: IVec3,
        end_grid_position: IVec3,
        path_request_id: Option<u64>,
        navigation: &mut Navigation,
        behavior: &mut Behavior,
    ) {
        if let Some(path_request_id) = path_request_id {
            if Navigation::poll_result(path_request_id, navigation) {
                if let Some(mut path_result) = Navigation::take_result(path_request_id, navigation)
                {
                    behavior.state = behavior::State::Moving {
                        path_vec: std::mem::take(&mut path_result.path_vec),
                    };
                }
            }
        } else {
            let path_request_id =
                Navigation::make_request(start_grid_position, end_grid_position, navigation);

            behavior.state = behavior::State::Navigating {
                grid_position: end_grid_position,
                path_request_id: Some(path_request_id),
            };
        }
    }

    fn handle_moving_state(path_vec: Vec<IVec3>, behavior: &mut Behavior) {}
}
