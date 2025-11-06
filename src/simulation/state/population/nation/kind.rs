use crate::simulation::state::world::block;
use glam::IVec3;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Kind {
    Eagle,
    Lion,
    Horse,
    Wolf,
}

impl Kind {
    pub fn all() -> [Kind; 4] {
        [Kind::Eagle, Kind::Lion, Kind::Horse, Kind::Wolf]
    }

    pub fn icon_block(&self) -> block::Kind {
        match self {
            Kind::Eagle => block::Kind::Icon2,
            Kind::Lion => block::Kind::Icon1,
            Kind::Horse => block::Kind::Icon4,
            Kind::Wolf => block::Kind::Icon3,
        }
    }

    pub fn home(&self) -> IVec3 {
        match self {
            Kind::Eagle => IVec3::new(-34, 2, 34),
            Kind::Lion => IVec3::new(34, 2, 34),
            Kind::Horse => IVec3::new(-34, 2, -34),
            Kind::Wolf => IVec3::new(34, 2, -34),
        }
    }

    pub fn color(&self) -> [f32; 4] {
        match self {
            Kind::Eagle => [0.65, 0.70, 0.80, 1.0],
            Kind::Lion => [0.70, 0.55, 0.85, 1.0],
            Kind::Horse => [0.988, 0.863, 0.592, 1.0],
            Kind::Wolf => [0.85, 0.35, 0.35, 1.0],
        }
    }

    pub fn from_string(string: &str) -> Option<Self> {
        if Self::matches_kind(string, "eagle") {
            Some(Kind::Eagle)
        } else if Self::matches_kind(string, "horse") {
            Some(Kind::Horse)
        } else if Self::matches_kind(string, "lion") {
            Some(Kind::Lion)
        } else if Self::matches_kind(string, "wolf") {
            Some(Kind::Wolf)
        } else {
            None
        }
    }

    fn matches_kind(string: &str, kind: &str) -> bool {
        string.to_ascii_lowercase().contains(kind)
    }
}
