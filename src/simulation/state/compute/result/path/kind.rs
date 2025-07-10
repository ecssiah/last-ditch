use crate::simulation::state::compute::result::path::data;

#[derive(Clone, Debug)]
pub enum Kind {
    Regional(data::Regional),
    Local(data::Local),
}
