use crate::simulation::{
    consts::*,
    state::{population::entity::Entity, Population},
};
use ultraviolet::Vec3;

pub fn run(population: &mut Population) {
    setup_judge(population);
}

fn setup_judge(population: &mut Population) {
    let judge = &mut population.judge;

    Entity::set_world_position(Vec3::new(0.0, 10.0, 0.0), &mut judge.entity);

    Entity::set_size(
        Vec3::new(JUDGE_SIZE_X, JUDGE_SIZE_Y, JUDGE_SIZE_Z),
        &mut judge.entity,
    );
}
