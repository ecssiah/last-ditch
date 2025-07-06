use crate::simulation::state::compute::result::{
    block_path_result::BlockPathResult, node_path_result::NodePathResult,
};

#[derive(Debug)]
pub enum Kind {
    NodePath(NodePathResult),
    BlockPath(BlockPathResult),
}
