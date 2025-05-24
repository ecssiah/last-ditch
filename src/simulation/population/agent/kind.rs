use crate::simulation::world::block;
use glam::IVec3;

#[derive(Clone, Debug)]
pub enum Kind {
    Lion,
    Eagle,
    Wolf,
    Horse,
}

impl Kind {
    pub fn get_list() -> [Kind; 4] {
        [Kind::Lion, Kind::Eagle, Kind::Wolf, Kind::Horse]
    }

    pub fn color(&self) -> [f32; 4] {
        match self {
            Kind::Lion => [0.70, 0.55, 0.85, 1.0],
            Kind::Eagle => [0.65, 0.70, 0.80, 1.0],
            Kind::Wolf => [0.85, 0.35, 0.35, 1.0],
            Kind::Horse => [0.988, 0.863, 0.592, 1.0],
        }
    }

    pub fn icon(&self) -> &block::Kind {
        match self {
            Kind::Lion => &block::Kind::Icon1,
            Kind::Eagle => &block::Kind::Icon2,
            Kind::Wolf => &block::Kind::Icon3,
            Kind::Horse => &block::Kind::Icon4,
        }
    }

    pub fn home(&self) -> IVec3 {
        match self {
            Kind::Lion => IVec3::new(34, 2, 34),
            Kind::Eagle => IVec3::new(-34, 2, 34),
            Kind::Wolf => IVec3::new(34, 2, -34),
            Kind::Horse => IVec3::new(-34, 2, -34),
        }
    }
}
