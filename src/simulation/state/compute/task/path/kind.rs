use crate::simulation::state::compute::task::path::data;

#[derive(Clone, Debug)]
pub enum Kind {
    Local(data::Local),
    Regional(data::Regional),
}
