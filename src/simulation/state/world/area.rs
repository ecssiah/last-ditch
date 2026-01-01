pub mod connection;
pub mod contact;
pub mod area_kind;
pub mod style;
pub mod template;

pub use connection::Connection;
pub use contact::Contact;
pub use area_kind::AreaKind;
pub use style::Style;

use crate::{
    simulation::state::world::grid::{self, Direction, Line},
    utils::ldmath::{ivec3_ext::rotate_by_direction, IntBox},
};
use ultraviolet::IVec3;

#[derive(Clone, Debug)]
pub struct Area {
    pub area_id: u64,
    pub area_kind: AreaKind,
    pub floor_number: i32,
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
            area_kind: AreaKind::UpperArea,
            floor_number: 0,
            style: Style::None,
            grid_position: IVec3::zero(),
            size: IVec3::one(),
            direction: Direction::North,
            connection_vec: Vec::new(),
        }
    }

    pub fn set_local(origin: IVec3, size: IVec3, area: &Self) -> IntBox {
        let local_int_box = IntBox::new(origin, origin + size - IVec3::one());
        let area_int_box = grid::get_grid_int_box(area.grid_position, area.size);

        let a = area_int_box.min + rotate_by_direction(local_int_box.min, area.direction);
        let b = area_int_box.min + rotate_by_direction(local_int_box.max, area.direction);

        IntBox::new(
            IVec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z)),
            IVec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z)),
        )
    }

    pub fn find_contact(area1: &Self, area2: &Self) -> Option<self::Contact> {
        let area1_int_box = grid::get_grid_int_box(area1.grid_position, area1.size);
        let area2_int_box = grid::get_grid_int_box(area2.grid_position, area2.size);

        if area1_int_box.max.x == area2_int_box.min.x || area2_int_box.max.x == area1_int_box.min.x
        {
            let x = if area1_int_box.max.x == area2_int_box.min.x {
                area1_int_box.max.x
            } else {
                area2_int_box.max.x
            };

            let y_overlap = Self::interval_overlap(
                area1_int_box.min.y,
                area1_int_box.max.y,
                area2_int_box.min.y,
                area2_int_box.max.y,
            )?;

            let z_overlap = Self::interval_overlap(
                area1_int_box.min.z,
                area1_int_box.max.z,
                area2_int_box.min.z,
                area2_int_box.max.z,
            )?;

            let contact = self::Contact::X {
                x,
                y_range: y_overlap,
                z_range: z_overlap,
            };

            return Some(contact);
        }

        if area1_int_box.max.y == area2_int_box.min.y || area2_int_box.max.y == area1_int_box.min.y
        {
            let y = if area1_int_box.max.y == area2_int_box.min.y {
                area1_int_box.max.y
            } else {
                area2_int_box.max.y
            };

            let x_overlap = Self::interval_overlap(
                area1_int_box.min.x,
                area1_int_box.max.x,
                area2_int_box.min.x,
                area2_int_box.max.x,
            )?;

            let z_overlap = Self::interval_overlap(
                area1_int_box.min.z,
                area1_int_box.max.z,
                area2_int_box.min.z,
                area2_int_box.max.z,
            )?;

            let contact = self::Contact::Y {
                y,
                x_range: x_overlap,
                z_range: z_overlap,
            };

            return Some(contact);
        }

        if area1_int_box.max.z == area2_int_box.min.z || area2_int_box.max.z == area1_int_box.min.z
        {
            let z = if area1_int_box.max.z == area2_int_box.min.z {
                area1_int_box.max.z
            } else {
                area2_int_box.max.z
            };

            let x_overlap = Self::interval_overlap(
                area1_int_box.min.x,
                area1_int_box.max.x,
                area2_int_box.min.x,
                area2_int_box.max.x,
            )?;

            let y_overlap = Self::interval_overlap(
                area1_int_box.min.y,
                area1_int_box.max.y,
                area2_int_box.min.y,
                area2_int_box.max.y,
            )?;

            let contact = self::Contact::Z {
                z,
                x_range: x_overlap,
                y_range: y_overlap,
            };

            return Some(contact);
        }

        None
    }

    pub fn find_ground_line(
        ground_level: i32,
        length_min: i32,
        contact: self::Contact,
    ) -> Option<Line> {
        match contact {
            self::Contact::X {
                x,
                y_range,
                z_range,
            } => {
                if ground_level < z_range.0 || ground_level > z_range.1 {
                    return None;
                }

                if y_range.1 - y_range.0 < length_min - 1 {
                    return None;
                }

                Some(Line::new(
                    IVec3::new(x, y_range.0, ground_level),
                    IVec3::new(x, y_range.1, ground_level),
                ))
            }
            self::Contact::Y {
                y,
                x_range,
                z_range,
            } => {
                if ground_level < z_range.0 || ground_level > z_range.1 {
                    return None;
                }

                if x_range.1 - x_range.0 < length_min - 1 {
                    return None;
                }

                Some(Line::new(
                    IVec3::new(x_range.0, y, ground_level),
                    IVec3::new(x_range.1, y, ground_level),
                ))
            }
            self::Contact::Z { .. } => None,
        }
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
