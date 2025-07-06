use crate::simulation::state::compute::task::{
    block_path_task::BlockPathTask, node_path_task::NodePathTask,
};

pub enum Kind {
    NodePath(NodePathTask),
    BlockPath(BlockPathTask),
}
