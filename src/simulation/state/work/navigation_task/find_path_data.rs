use crate::simulation::state::navigation::{path, Navigation};
use egui::ahash::HashSet;
use std::collections::{BinaryHeap, HashMap};
use ultraviolet::IVec3;

#[derive(Clone)]
pub struct FindPathData {
    pub start: IVec3,
    pub end: IVec3,
    pub open_heap: BinaryHeap<path::Node>,
    pub closed_set: HashSet<IVec3>,
    pub origin_map: HashMap<IVec3, IVec3>,
}

impl FindPathData {
    pub fn cost(_find_path_data: &Self) -> u32 {
        0
    }

    pub fn step(_navigation: &Navigation, _find_path_data: &mut Self) -> bool {
        true
    }

    // fn next_stage(generation_data: &mut GenerationData) -> bool {
    //     generation_data.stage += 1;

    //     generation_data.stage >= generation_data.stage_cost_vec.len()
    // }
}
