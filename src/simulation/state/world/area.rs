pub mod connection;
pub mod contact;
pub mod kind;
pub mod style;
pub mod template;

pub use connection::Connection;
pub use contact::Contact;
pub use kind::Kind;
pub use style::Style;

use crate::{
    simulation::state::world::grid::{self, Direction, Line},
    utils::ldmath::{ivec3_ext::rotate_by_direction, IBox},
};
use ultraviolet::IVec3;

#[derive(Clone, Debug)]
pub struct Area {
    pub area_id: u64,
    pub floor_number: i32,
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
            floor_number: -1,
            kind: Kind::UpperArea,
            style: Style::None,
            grid_position: IVec3::zero(),
            size: IVec3::one(),
            direction: Direction::North,
            connection_vec: Vec::new(),
        }
    }

    pub fn set_local(origin: IVec3, size: IVec3, area: &Self) -> IBox {
        let local_ibox = IBox::new(origin, origin + size - IVec3::one());
        let area_ibox = grid::get_grid_ibox(area.grid_position, area.size);

        let a = area_ibox.min + rotate_by_direction(local_ibox.min, area.direction);
        let b = area_ibox.min + rotate_by_direction(local_ibox.max, area.direction);

        IBox::new(
            IVec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z)),
            IVec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z)),
        )
    }

    pub fn find_contact(area1: &Self, area2: &Self) -> Option<Contact> {
        let area1_ibox = grid::get_grid_ibox(area1.grid_position, area1.size);
        let area2_ibox = grid::get_grid_ibox(area2.grid_position, area2.size);

        if area1_ibox.max.x == area2_ibox.min.x || area2_ibox.max.x == area1_ibox.min.x {
            let x = if area1_ibox.max.x == area2_ibox.min.x {
                area1_ibox.max.x
            } else {
                area2_ibox.max.x
            };

            let y_overlap = Self::interval_overlap(
                area1_ibox.min.y,
                area1_ibox.max.y,
                area2_ibox.min.y,
                area2_ibox.max.y,
            )?;

            let z_overlap = Self::interval_overlap(
                area1_ibox.min.z,
                area1_ibox.max.z,
                area2_ibox.min.z,
                area2_ibox.max.z,
            )?;

            let contact = Contact::X {
                x,
                y_range: y_overlap,
                z_range: z_overlap,
            };

            return Some(contact);
        }

        if area1_ibox.max.y == area2_ibox.min.y || area2_ibox.max.y == area1_ibox.min.y {
            let y = if area1_ibox.max.y == area2_ibox.min.y {
                area1_ibox.max.y
            } else {
                area2_ibox.max.y
            };

            let x_overlap = Self::interval_overlap(
                area1_ibox.min.x,
                area1_ibox.max.x,
                area2_ibox.min.x,
                area2_ibox.max.x,
            )?;

            let z_overlap = Self::interval_overlap(
                area1_ibox.min.z,
                area1_ibox.max.z,
                area2_ibox.min.z,
                area2_ibox.max.z,
            )?;

            let contact = Contact::Y {
                y,
                x_range: x_overlap,
                z_range: z_overlap,
            };

            return Some(contact);
        }

        if area1_ibox.max.z == area2_ibox.min.z || area2_ibox.max.z == area1_ibox.min.z {
            let z = if area1_ibox.max.z == area2_ibox.min.z {
                area1_ibox.max.z
            } else {
                area2_ibox.max.z
            };

            let x_overlap = Self::interval_overlap(
                area1_ibox.min.x,
                area1_ibox.max.x,
                area2_ibox.min.x,
                area2_ibox.max.x,
            )?;

            let y_overlap = Self::interval_overlap(
                area1_ibox.min.y,
                area1_ibox.max.y,
                area2_ibox.min.y,
                area2_ibox.max.y,
            )?;

            let contact = Contact::Z {
                z,
                x_range: x_overlap,
                y_range: y_overlap,
            };

            return Some(contact);
        }

        None
    }

    pub fn find_ground_line(ground_level: i32, length_min: i32, contact: Contact) -> Option<Line> {
        match contact {
            Contact::X {
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
            Contact::Y {
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
            Contact::Z { .. } => None,
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
