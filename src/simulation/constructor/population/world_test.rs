use crate::simulation::{
    consts::*,
    state::{Population, World},
};
use glam::Vec3;

pub fn construct(population: &mut Population, _world: &World) {
    setup_judge(population);
}

fn setup_judge(population: &mut Population) {
    let judge = &mut population.judge;

    judge.set_world_position(Vec3::new(0.0, -2.0, 0.0));
    judge.set_size(Vec3::new(JUDGE_SIZE_X, JUDGE_SIZE_Y, JUDGE_SIZE_Z));
}
