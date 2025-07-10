pub mod path;

use crate::simulation::state::{compute, world::graph::Graph};

#[derive(Clone, Debug)]
pub enum Task {
    Path(path::Kind),
}

impl Task {
    pub fn execute(task: &mut Task) -> compute::Result {
        match task {
            Task::Path(ref mut kind) => match kind {
                path::Kind::Local(ref mut task_data) => {
                    let result_data = compute::result::path::data::Local {
                        agent_id: task_data.agent_id,
                        chunk_id: task_data.chunk_id,
                        position_vec: Vec::new(),
                    };

                    let result =
                        compute::Result::Path(compute::result::path::Kind::Local(result_data));

                    result
                }
                path::Kind::Regional(ref mut task_data) => {
                    let node_vec = Graph::find_path(
                        task_data.start_position,
                        task_data.end_position,
                        &task_data.level_0,
                        &mut task_data.search_level,
                    );

                    let position_vec = node_vec.iter().map(|node| node.position).collect();

                    let result_data = compute::result::path::data::Regional {
                        agent_id: task_data.agent_id,
                        position_vec,
                    };

                    let result =
                        compute::Result::Path(compute::result::path::Kind::Regional(result_data));

                    result
                }
            },
        }
    }
}
