use crate::simulation::state::Population;

pub enum PopulationTask {
    Construct,
}

impl PopulationTask {
    pub fn step(_population: &mut Population, _world_task: &mut PopulationTask) -> bool {
        false
    }
}
