use crate::simulation::state::world::grid::Direction;
use std::fmt;

#[derive(Clone, Copy, Debug, Default)]
pub struct DirectionSet(u8);

impl DirectionSet {
    pub const EMPTY: Self = Self(0);

    pub const ALL: Self = Self(
        Direction::North as u8
            | Direction::West as u8
            | Direction::South as u8
            | Direction::East as u8
            | Direction::Up as u8
            | Direction::Down as u8,
    );

    pub fn has(direction: Direction, direction_set: &DirectionSet) -> bool {
        (direction_set.0 & direction as u8) != 0
    }

    pub fn add(direction: Direction, direction_set: &mut DirectionSet) {
        direction_set.0 |= direction as u8;
    }

    pub fn remove(direction: Direction, direction_set: &mut DirectionSet) {
        direction_set.0 &= !(direction as u8);
    }

    pub fn clear(direction_set: &mut DirectionSet) {
        direction_set.0 = 0;
    }
}

impl fmt::Display for DirectionSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;

        for direction in Direction::ALL {
            if Self::has(*direction, self) {
                if !first {
                    f.write_str(", ")?;
                }

                f.write_str(direction.as_str())?;
                first = false;
            }
        }

        if first {
            f.write_str("none")?;
        }

        Ok(())
    }
}
