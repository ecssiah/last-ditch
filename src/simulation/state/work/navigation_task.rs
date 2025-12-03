pub mod find_path_data;

use crate::simulation::state::{
    navigation::Navigation, work::navigation_task::find_path_data::FindPathData,
};

#[derive(Clone)]
pub enum NavigationTask {
    FindPath(FindPathData),
}

impl NavigationTask {
    pub fn step(_navigation: &mut Navigation, _navigation_task: &mut Self) -> bool {
        false
    }
}
