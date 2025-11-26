use crate::simulation::{
    constants::*,
    state::{population::spatial::Spatial, Population},
};
use ultraviolet::Vec3;

pub fn construct(population: &mut Population) {
    setup_judge(population);
}

fn setup_judge(population: &mut Population) {
    let judge = &mut population.judge;

    Spatial::set_world_position(Vec3::new(0.0, 10.0, 0.0), &mut judge.spatial);

    Spatial::set_size(
        Vec3::new(
            JUDGE_DEFAULT_SIZE_X,
            JUDGE_DEFAULT_SIZE_Y,
            JUDGE_DEFAULT_SIZE_Z,
        ),
        &mut judge.spatial,
    );
}
