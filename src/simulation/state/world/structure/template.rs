use crate::simulation::state::world::{block, grid};
use ultraviolet::IVec3;

pub fn trade_platform(direction: grid::Direction) -> Vec<(block::Kind, IVec3)> {
    #[rustfmt::skip]
    let template = vec![
        (block::Kind::Metal3,       IVec3::new(-2, -1,  0)),
        (block::Kind::None,         IVec3::new(-1, -1,  0)),
        (block::Kind::None,         IVec3::new( 0, -1,  0)),
        (block::Kind::None,         IVec3::new( 1, -1,  0)),
        (block::Kind::Metal3,       IVec3::new( 2, -1,  0)),

        (block::Kind::Metal3,       IVec3::new(-2,  0,  0)),
        (block::Kind::None,         IVec3::new(-1,  0,  0)),
        (block::Kind::None,         IVec3::new( 0,  0,  0)),
        (block::Kind::None,         IVec3::new( 1,  0,  0)),
        (block::Kind::Metal3,       IVec3::new( 2,  0,  0)),

        (block::Kind::Metal3,       IVec3::new(-2,  1,  0)),
        (block::Kind::None,         IVec3::new(-1,  1,  0)),
        (block::Kind::None,         IVec3::new( 0,  1,  0)),
        (block::Kind::None,         IVec3::new( 1,  1,  0)),
        (block::Kind::Metal3,       IVec3::new( 2,  1,  0)),

        (block::Kind::Metal1,       IVec3::new(-2,  0, -1)),
        (block::Kind::Metal1,       IVec3::new(-1,  0, -1)),
        (block::Kind::Metal1,       IVec3::new( 0,  0, -1)),
        (block::Kind::Metal1,       IVec3::new( 1,  0, -1)),
        (block::Kind::Metal1,       IVec3::new( 2,  0, -1)),

        (block::Kind::Metal1,       IVec3::new(-2,  1, -1)),
        (block::Kind::Metal1,       IVec3::new(-1,  1, -1)),
        (block::Kind::Metal1,       IVec3::new( 0,  1, -1)),
        (block::Kind::Metal1,       IVec3::new( 1,  1, -1)),
        (block::Kind::Metal1,       IVec3::new( 2,  1, -1)),

        (block::Kind::Metal1,       IVec3::new(-2,  2, -1)),
        (block::Kind::Metal1,       IVec3::new(-1,  2, -1)),
        (block::Kind::Metal1,       IVec3::new( 0,  2, -1)),
        (block::Kind::Metal1,       IVec3::new( 1,  2, -1)),
        (block::Kind::Metal1,       IVec3::new( 2,  2, -1)),

        (block::Kind::Metal1,       IVec3::new(-2,  3, -1)),
        (block::Kind::Metal1,       IVec3::new(-1,  3, -1)),
        (block::Kind::Metal1,       IVec3::new( 0,  3, -1)),
        (block::Kind::Metal1,       IVec3::new( 1,  3, -1)),
        (block::Kind::Metal1,       IVec3::new( 2,  3, -1)),
        
        (block::Kind::Metal1,       IVec3::new(-2,  4, -1)),
        (block::Kind::Metal1,       IVec3::new(-1,  4, -1)),
        (block::Kind::Metal1,       IVec3::new( 0,  4, -1)),
        (block::Kind::Metal1,       IVec3::new( 1,  4, -1)),
        (block::Kind::Metal1,       IVec3::new( 2,  4, -1)),
    ];

    rotate_template(template, direction)
}

fn rotate_template(
    template: Vec<(block::Kind, IVec3)>,
    direction: grid::Direction,
) -> Vec<(block::Kind, IVec3)> {
    match direction {
        grid::Direction::East => template
            .into_iter()
            .map(|(block_kind, grid_position)| {
                (
                    block_kind,
                    IVec3::new(grid_position.y, grid_position.x, grid_position.z),
                )
            })
            .collect(),
        grid::Direction::West => template
            .into_iter()
            .map(|(block_kind, grid_position)| {
                (
                    block_kind,
                    IVec3::new(-grid_position.y, -grid_position.x, grid_position.z),
                )
            })
            .collect(),
        grid::Direction::North => template,
        grid::Direction::South => template
            .into_iter()
            .map(|(block_kind, grid_position)| {
                (
                    block_kind,
                    IVec3::new(grid_position.x, -grid_position.y, grid_position.z),
                )
            })
            .collect(),
        _ => template,
    }
}
