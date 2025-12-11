use crate::simulation::state::world::grid::{Connection, Line};
use ultraviolet::IVec3;

#[derive(Clone, Debug)]
pub struct Area {
    pub area_id: u64,
    pub grid_position: IVec3,
    pub size: IVec3,
    pub connection_vec: Vec<Connection>,
}

impl Area {
    fn min(area: &Self) -> IVec3 {
        area.grid_position
    }

    fn max(area: &Self) -> IVec3 {
        area.grid_position + area.size
    }

    pub fn find_shared_line(area1: &Self, area2: &Self) -> Option<Line> {
        let area1_min = Area::min(area1);
        let area2_min = Area::min(area2);
        let area1_max = Area::max(area1);
        let area2_max = Area::max(area2);

        // Check if they touch on X face
        if area1_max.x == area2_min.x || area2_max.x == area1_min.x {
            // They meet at plane x = const
            let x = if area1_max.x == area2_min.x {
                area1_max.x
            } else {
                area2_max.x
            };

            let y_overlap =
                Self::interval_overlap(area1_min.y, area1_max.y, area2_min.y, area2_max.y)?;
            let z_overlap =
                Self::interval_overlap(area1_min.z, area1_max.z, area2_min.z, area2_max.z)?;

            // Shared line is along Y or Z depending on overlap
            // But for an edge, only one dimension should be non-degenerate.
            // Choose the longest non-empty segment between the two dims.

            if y_overlap.0 < y_overlap.1 {
                return Some(Line {
                    grid_position1: IVec3::new(x, y_overlap.0, z_overlap.0),
                    grid_position2: IVec3::new(x, y_overlap.1, z_overlap.0),
                });
            }

            if z_overlap.0 < z_overlap.1 {
                return Some(Line {
                    grid_position1: IVec3::new(x, y_overlap.0, z_overlap.0),
                    grid_position2: IVec3::new(x, y_overlap.0, z_overlap.1),
                });
            }
        }

        // Check if they touch on Y face
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

            if x_overlap.0 < x_overlap.1 {
                return Some(Line {
                    grid_position1: IVec3::new(x_overlap.0, y, z_overlap.0),
                    grid_position2: IVec3::new(x_overlap.1, y, z_overlap.0),
                });
            }

            if z_overlap.0 < z_overlap.1 {
                return Some(Line {
                    grid_position1: IVec3::new(x_overlap.0, y, z_overlap.0),
                    grid_position2: IVec3::new(x_overlap.0, y, z_overlap.1),
                });
            }
        }

        // Check if they touch on Z face
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

            if x_overlap.0 < x_overlap.1 {
                return Some(Line {
                    grid_position1: IVec3::new(x_overlap.0, y_overlap.0, z),
                    grid_position2: IVec3::new(x_overlap.1, y_overlap.0, z),
                });
            }

            if y_overlap.0 < y_overlap.1 {
                return Some(Line {
                    grid_position1: IVec3::new(x_overlap.0, y_overlap.0, z),
                    grid_position2: IVec3::new(x_overlap.0, y_overlap.1, z),
                });
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
