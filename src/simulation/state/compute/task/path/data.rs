pub mod local;
pub mod regional;

pub use local::Local;
pub use regional::Regional;

use crate::simulation::state::compute::task::path::data;

#[derive(Clone, Debug)]
pub enum Data {
    Local(data::Local),
    Regional(data::Regional),
}
