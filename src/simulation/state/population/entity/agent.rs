use crate::simulation::state::{
    population::{
        entity::{self, Detection, Info, Kinematic, Spatial},
        nation,
    },
    world::{sector, World},
};
use ultraviolet::Vec3;

pub struct Agent {
    pub info: Info,
    pub spatial: Spatial,
    pub kinematic: Kinematic,
    pub detection: Detection,
}

impl Agent {
    pub fn new(nation_kind: nation::Kind) -> Self {
        let info = Info {
            entity_id: entity::ID::allocate(),
            sector_id: sector::ID(0),
            sector_updated: false,
            entity_kind: entity::Kind::Agent,
            nation_kind,
        };

        Self {
            info,
            spatial: Spatial::new(),
            kinematic: Kinematic::new(),
            detection: Detection::new(),
        }
    }

    pub fn set_world_position(
        world_position: Vec3,
        spatial: &mut Spatial,
        detection: &mut Detection,
    ) {
        spatial.world_position = world_position;

        Detection::set_world_position(world_position, detection);
    }

    pub fn set_size(size: Vec3, spatial: &mut Spatial, detection: &mut Detection) {
        spatial.size = size;

        Detection::set_size(size, detection);
    }

    pub fn tick(world: &World, agent: &mut Agent) {
        Info::tick(world, &agent.spatial, &mut agent.info);
    }
}
