use crate::simulation::state::world::grid::{Connection, Line};
use ultraviolet::IVec3;

#[derive(Clone, Debug)]
pub struct Area {
    pub area_id: u64,
    pub min: IVec3,
    pub max: IVec3,
    pub connection_vec: Vec<Connection>,
}

impl Area {
    pub fn size(area: &Self) -> IVec3 {
        area.max - area.min
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

            if y_overlap.0 < y_overlap.1 {
                return Some(Line::new(
                    IVec3::new(x, y_overlap.0, z_overlap.0),
                    IVec3::new(x, y_overlap.1, z_overlap.0),
                ));
            }

            if z_overlap.0 < z_overlap.1 {
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

            if x_overlap.0 < x_overlap.1 {
                return Some(Line::new(
                    IVec3::new(x_overlap.0, y, z_overlap.0),
                    IVec3::new(x_overlap.1, y, z_overlap.0),
                ));
            }

            if z_overlap.0 < z_overlap.1 {
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

            if x_overlap.0 < x_overlap.1 {
                return Some(Line::new(
                    IVec3::new(x_overlap.0, y_overlap.0, z),
                    IVec3::new(x_overlap.1, y_overlap.0, z),
                ));
            }

            if y_overlap.0 < y_overlap.1 {
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
        let end = (a1).min(b1);

        if start < end {
            Some((start, end))
        } else {
            None
        }
    }
}
