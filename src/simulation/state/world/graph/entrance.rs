use crate::simulation::state::world::graph::Transition;
use glam::{IVec3, Vec3};
use std::collections::HashSet;

#[derive(Clone)]
pub struct Entrance {
    pub region_position1: IVec3,
    pub region_position2: IVec3,
    pub transition_vec: Vec<Transition>,
}

impl Entrance {
    pub fn new(region_position1: IVec3, region_position2: IVec3) -> Self {
        Self {
            region_position1,
            region_position2,
            transition_vec: Vec::new(),
        }
    }

    pub fn representative_transitions(&self) -> Vec<Transition> {
        let mut candidates = Vec::new();

        if let Some(min_x_transition) = self
            .transition_vec
            .iter()
            .min_by_key(|transition| transition.position1.x)
        {
            candidates.push(*min_x_transition);
        }

        if let Some(max_x_transition) = self
            .transition_vec
            .iter()
            .max_by_key(|transition| transition.position1.x)
        {
            candidates.push(*max_x_transition);
        }

        if let Some(min_y_transition) = self
            .transition_vec
            .iter()
            .min_by_key(|transition| transition.position1.y)
        {
            candidates.push(*min_y_transition);
        }

        if let Some(max_y_transition) = self
            .transition_vec
            .iter()
            .max_by_key(|transition| transition.position1.y)
        {
            candidates.push(*max_y_transition);
        }

        if let Some(min_z_transition) = self
            .transition_vec
            .iter()
            .min_by_key(|transition| transition.position1.z)
        {
            candidates.push(*min_z_transition);
        }

        if let Some(max_z_transition) = self
            .transition_vec
            .iter()
            .max_by_key(|transition| transition.position1.z)
        {
            candidates.push(*max_z_transition);
        }

        let position_sum = self
            .transition_vec
            .iter()
            .map(|transition| transition.position1.as_vec3())
            .fold(Vec3::ZERO, |position_sum, position| position_sum + position);

        let transition_count = self.transition_vec.len().max(1) as f32;

        let average_transition_position = position_sum / transition_count;

        let center_transition = self.transition_vec.iter().min_by_key(|transition| {
            let delta = transition.position1.as_vec3() - average_transition_position;
            let delta_length_squared = (delta.length_squared() * 1000.0) as i32;

            delta_length_squared
        });

        if let Some(center_transition) = center_transition {
            candidates.push(*center_transition);
        }

        let mut seen = HashSet::new();
        candidates.retain(|transition| seen.insert((transition.position1, transition.position2)));

        candidates
    }
}
