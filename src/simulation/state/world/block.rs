pub mod kind;

pub use kind::Kind;

use crate::simulation::{
    constants::CELL_RADIUS_IN_METERS,
    state::world::{grid::direction_set::DirectionSet, object},
};
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct Block {
    pub block_kind: self::Kind,
    pub solid: bool,
    pub exposure_set: DirectionSet,
}

impl Block {
    pub fn new(block_kind: &self::Kind) -> Self {
        let block_info = Self::get_block_info(block_kind);
        let exposure_set = DirectionSet::EMPTY;

        Self {
            block_kind: block_kind.clone(),
            solid: block_info.solid,
            exposure_set,
        }
    }

    pub fn get_block_info(block_kind: &self::Kind) -> object::Info {
        match block_kind {
            self::Kind::Engraved1 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Engraved2 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Engraved3 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Engraved4 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Ornate1 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Ornate2 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Ornate3 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Ornate4 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::CarvedStone1 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::CarvedStone2 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::CarvedStone3 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::CarvedStone4 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Stone1 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Stone2 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Stone3 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Stone4 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Lion => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Eagle => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Horse => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Wolf => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::LionStone => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::EagleStone => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::HorseStone => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::WolfStone => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::NorthBlock => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::WestBlock => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::SouthBlock => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::EastBlock => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Server1 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Server2 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Server3 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Server4 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Metal1 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Metal2 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Metal3 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Metal4 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Panel1 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Panel2 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Panel3 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Vent1 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Vent2 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Vent3 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Vent4 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
            self::Kind::Caution => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::broadcast(CELL_RADIUS_IN_METERS),
            },
        }
    }
}
