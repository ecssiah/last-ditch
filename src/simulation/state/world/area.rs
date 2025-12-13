pub mod connection;
pub mod kind;
pub mod style;
pub mod template;

pub use connection::Connection;
pub use kind::Kind;
pub use style::Style;

use crate::{
    simulation::state::world::grid::{Direction, Line},
    utils::ldmath::ivec3_ext::rotate_by_direction,
};
use ultraviolet::IVec3;

#[derive(Clone, Debug)]
pub struct Area {
    pub area_id: u64,
    pub kind: Kind,
    pub style: Style,
    pub min: IVec3,
    pub max: IVec3,
    pub direction: Direction,
    pub connection_vec: Vec<Connection>,
}

impl Area {
    pub fn size(area: &Self) -> IVec3 {
        area.max - area.min
    }

    pub fn local_to_world_bounds(
        min_offset: IVec3,
        max_offset: IVec3,
        area: &Self,
    ) -> (IVec3, IVec3) {
        let size = area.max - area.min;

        let min_offset_rotated = rotate_by_direction(min_offset, area.direction);
        let max_offset_rotated = rotate_by_direction(size + max_offset, area.direction);

        let min_world = area.min + min_offset_rotated;
        let max_world = area.min + max_offset_rotated;

        let min = IVec3::new(
            min_world.x.min(max_world.x),
            min_world.y.min(max_world.y),
            min_world.z.min(max_world.z),
        );

        let max = IVec3::new(
            min_world.x.max(max_world.x),
            min_world.y.max(max_world.y),
            min_world.z.max(max_world.z),
        );

        (min, max)
    }

    pub fn find_shared_line(area1: &Self, area2: &Self) -> Option<Line> {
        if area1.max.x == area2.min.x || area2.max.x == area1.min.x {
            let x = if area1.max.x == area2.min.x {
                area1.max.x
            } else {
                area2.max.x
            };

            let y_overlap =
                Self::interval_overlap(area1.min.y, area1.max.y, area2.min.y, area2.max.y)?;

            let z_overlap =
                Self::interval_overlap(area1.min.z, area1.max.z, area2.min.z, area2.max.z)?;

            if (y_overlap.1 - y_overlap.0) >= (z_overlap.1 - z_overlap.0) {
                return Some(Line::new(
                    IVec3::new(x, y_overlap.0, z_overlap.0),
                    IVec3::new(x, y_overlap.1, z_overlap.0),
                ));
            } else {
                return Some(Line::new(
                    IVec3::new(x, y_overlap.0, z_overlap.0),
                    IVec3::new(x, y_overlap.0, z_overlap.1),
                ));
            }
        }

        if area1.max.y == area2.min.y || area2.max.y == area1.min.y {
            let y = if area1.max.y == area2.min.y {
                area1.max.y
            } else {
                area2.max.y
            };

            let x_overlap =
                Self::interval_overlap(area1.min.x, area1.max.x, area2.min.x, area2.max.x)?;

            let z_overlap =
                Self::interval_overlap(area1.min.z, area1.max.z, area2.min.z, area2.max.z)?;

            if (x_overlap.1 - x_overlap.0) >= (z_overlap.1 - z_overlap.0) {
                return Some(Line::new(
                    IVec3::new(x_overlap.0, y, z_overlap.0),
                    IVec3::new(x_overlap.1, y, z_overlap.0),
                ));
            } else {
                return Some(Line::new(
                    IVec3::new(x_overlap.0, y, z_overlap.0),
                    IVec3::new(x_overlap.0, y, z_overlap.1),
                ));
            }
        }

        if area1.max.z == area2.min.z || area2.max.z == area1.min.z {
            let z = if area1.max.z == area2.min.z {
                area1.max.z
            } else {
                area2.max.z
            };

            let x_overlap =
                Self::interval_overlap(area1.min.x, area1.max.x, area2.min.x, area2.max.x)?;

            let y_overlap =
                Self::interval_overlap(area1.min.y, area1.max.y, area2.min.y, area2.max.y)?;

            if (x_overlap.1 - x_overlap.0) >= (y_overlap.1 - y_overlap.0) {
                return Some(Line::new(
                    IVec3::new(x_overlap.0, y_overlap.0, z),
                    IVec3::new(x_overlap.1, y_overlap.0, z),
                ));
            } else {
                return Some(Line::new(
                    IVec3::new(x_overlap.0, y_overlap.0, z),
                    IVec3::new(x_overlap.0, y_overlap.1, z),
                ));
            }
        }

        None
    }

    fn interval_overlap(a0: i32, a1: i32, b0: i32, b1: i32) -> Option<(i32, i32)> {
        let start = a0.max(b0);
        let end = a1.min(b1);

        if start <= end {
            Some((start, end))
        } else {
            None
        }
    }
}
