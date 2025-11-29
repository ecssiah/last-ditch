use crate::simulation::state::navigation::Navigation;

pub enum NavigationTask {}

impl NavigationTask {
    pub fn step(_navigation: &mut Navigation, _navigation_task: &mut NavigationTask) -> bool {
        false
    }
}
