pub mod connection;
pub mod kind;
pub mod style;
pub mod template;

pub use connection::Connection;
pub use kind::Kind;
pub use style::Style;

use crate::{
    simulation::state::world::grid::{self, Direction, Line},
    utils::ldmath::ivec3_ext::rotate_by_direction,
};
use ultraviolet::IVec3;

#[derive(Clone, Debug)]
pub struct Area {
    pub area_id: u64,
    pub kind: Kind,
    pub style: Style,
    pub grid_position: IVec3,
    pub size: IVec3,
    pub direction: Direction,
    pub connection_vec: Vec<Connection>,
}

impl Area {
    pub fn new(area_id: u64) -> Self {
        Self {
            area_id,
            kind: Kind::UpperArea,
            style: Style::None,
            grid_position: IVec3::zero(),
            size: IVec3::one(),
            direction: Direction::North,
            connection_vec: Vec::new(),
        }
    }

    pub fn set_local(origin: IVec3, size: IVec3, area: &Self) -> (IVec3, IVec3) {
        let local_min = origin;
        let local_max = origin + size - IVec3::one();

        let (area_min, _) = grid::get_bounds(area.grid_position, area.size);

        let a = area_min + rotate_by_direction(local_min, area.direction);
        let b = area_min + rotate_by_direction(local_max, area.direction);

        let min = IVec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
        let max = IVec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

        (min, max)
    }

    pub fn find_shared_line(area1: &Self, area2: &Self) -> Option<Line> {
        let (area1_min, area1_max) = grid::get_bounds(area1.grid_position, area1.size);
        let (area2_min, area2_max) = grid::get_bounds(area2.grid_position, area2.size);

        if area1_max.x == area2_min.x || area2_max.x == area1_min.x {
            let x = if area1_max.x == area2_min.x {
                area1_max.x
            } else {
                area2_max.x
            };

            let y_overlap =
                Self::interval_overlap(area1_min.y, area1_max.y, area2_min.y, area2_max.y)?;

            let z_overlap =
                Self::interval_overlap(area1_min.z, area1_max.z, area2_min.z, area2_max.z)?;

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

        if area1_max.y == area2_min.y || area2_max.y == area1_min.y {
            let y = if area1_max.y == area2_min.y {
                area1_max.y
            } else {
                area2_max.y
            };

            let x_overlap =
                Self::interval_overlap(area1_min.x, area1_max.x, area2_min.x, area2_max.x)?;

            let z_overlap =
                Self::interval_overlap(area1_min.z, area1_max.z, area2_min.z, area2_max.z)?;

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

        if area1_max.z == area2_min.z || area2_max.z == area1_min.z {
            let z = if area1_max.z == area2_min.z {
                area1_max.z
            } else {
                area2_max.z
            };

            let x_overlap =
                Self::interval_overlap(area1_min.x, area1_max.x, area2_min.x, area2_max.x)?;

            let y_overlap =
                Self::interval_overlap(area1_min.y, area1_max.y, area2_min.y, area2_max.y)?;

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
