use crate::simulation::constants::{
    WORLD_RADIUS_IN_CELLS, WORLD_SIZE_IN_CELLS, WORLD_VOLUME_IN_CELLS,
};
use ultraviolet::IVec3;

pub struct Graph {
    solid_vec: Vec<bool>,
    cost_vec: Vec<u8>,
}

impl Graph {
    #[rustfmt::skip]
    pub const NEIGHBOR_OFFSETS: [IVec3; 26] = [
        IVec3::new(-1, -1, -1),
        IVec3::new( 0, -1, -1),
        IVec3::new( 1, -1, -1),
        IVec3::new(-1,  0, -1),
        IVec3::new( 0,  0, -1),
        IVec3::new( 1,  0, -1),
        IVec3::new(-1,  1, -1),
        IVec3::new( 0,  1, -1),
        IVec3::new( 1,  1, -1),
        IVec3::new(-1, -1,  0),
        IVec3::new( 0, -1,  0),
        IVec3::new( 1, -1,  0),
        IVec3::new(-1,  0,  0),
        IVec3::new( 1,  0,  0),
        IVec3::new(-1,  1,  0),
        IVec3::new( 0,  1,  0),
        IVec3::new( 1,  1,  0),
        IVec3::new(-1, -1,  1),
        IVec3::new( 0, -1,  1),
        IVec3::new( 1, -1,  1),
        IVec3::new(-1,  0,  1),
        IVec3::new( 0,  0,  1),
        IVec3::new( 1,  0,  1),
        IVec3::new(-1,  1,  1),
        IVec3::new( 0,  1,  1),
        IVec3::new( 1,  1,  1),
    ];

    pub fn new() -> Self {
        let solid_vec = vec![false; WORLD_VOLUME_IN_CELLS];
        let cost_vec = vec![1u8; WORLD_VOLUME_IN_CELLS];

        Self {
            solid_vec,
            cost_vec,
        }
    }

    pub fn get_index(position: IVec3) -> usize {
        let position_indexable = position + IVec3::broadcast(WORLD_RADIUS_IN_CELLS as i32);

        ((position_indexable.z as usize * WORLD_SIZE_IN_CELLS + position_indexable.y as usize)
            * WORLD_SIZE_IN_CELLS
            + position_indexable.x as usize) as usize
    }

    pub fn grid_position_is_valid(position: IVec3) -> bool {
        let world_radius_in_cells = WORLD_RADIUS_IN_CELLS as i32;

        position.x >= -world_radius_in_cells
            && position.x <= world_radius_in_cells
            && position.y >= -world_radius_in_cells
            && position.y <= world_radius_in_cells
            && position.z >= -world_radius_in_cells
            && position.z <= world_radius_in_cells
    }

    #[inline]
    pub fn is_walkable(position: IVec3, graph: &Self) -> bool {
        if !Self::grid_position_is_valid(position) {
            return false;
        }

        let index = Self::get_index(position);

        if graph.solid_vec[index] {
            return false;
        }

        let position_below = position - IVec3::unit_z();

        if !Self::grid_position_is_valid(position_below) {
            return false;
        }

        let index_below = Self::get_index(position_below);

        let is_solid_ground = graph.solid_vec[index_below];

        is_solid_ground
    }

    pub fn set_solid(position: IVec3, is_solid: bool, graph: &mut Self) {
        if Self::grid_position_is_valid(position) {
            let index = Self::get_index(position);

            graph.solid_vec[index] = is_solid;
        }
    }

    #[inline]
    pub fn get_cost(position: IVec3, graph: &Self) -> i32 {
        let index = Self::get_index(position);

        graph.cost_vec[index] as i32
    }

    #[inline]
    pub fn get_valid_neighbor_position_iter(
        position: IVec3,
        graph: &Self,
    ) -> impl Iterator<Item = IVec3> {
        let mut open_neighbor_position_vec = Vec::with_capacity(Self::NEIGHBOR_OFFSETS.len());

        for neighbor_offset in Self::NEIGHBOR_OFFSETS.iter() {
            let neighbor_position = position + *neighbor_offset;

            if Self::grid_position_is_valid(neighbor_position)
                && Self::is_walkable(neighbor_position, graph)
            {
                open_neighbor_position_vec.push(neighbor_position);
            }
        }

        open_neighbor_position_vec.into_iter()
    }
}
