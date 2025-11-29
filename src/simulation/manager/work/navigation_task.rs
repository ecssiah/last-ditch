use crate::simulation::state::navigation::Navigation;

#[derive(Clone)]
pub enum NavigationTask {}

impl NavigationTask {
    pub fn step(_navigation: &mut Navigation, _navigation_task: &mut NavigationTask) -> bool {
        false
    }
}
