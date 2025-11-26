//! Current state of the simulation

pub mod action;
pub mod admin;
pub mod config;
pub mod navigation;
pub mod physics;
pub mod population;
pub mod receiver;
pub mod state_loading;
pub mod state_menu;
pub mod state_shutdown;
pub mod state_simulate;
pub mod template;
pub mod time;
pub mod world;

pub use action::Action;
pub use admin::Admin;
pub use config::Config;
pub use physics::Physics;
pub use population::Population;
pub use receiver::Receiver;
pub use template::Template;
pub use time::Time;
use ultraviolet::{IVec3, Vec3};
pub use world::World;

use crate::simulation::state::{
    self,
    navigation::{Graph, Navigation},
    world::block,
};

pub struct State {
    pub active: bool,
    pub template: state::Template,
    pub construct_rx: Option<tokio::sync::mpsc::Receiver<(World, Population)>>,
    pub admin: Admin,
    pub action: Action,
    pub world: World,
    pub population: Population,
    pub physics: Physics,
    pub navigation: Navigation,
    pub time: Time,
}

impl State {
    pub fn new() -> Self {
        let active = true;
        let template = state::Template::Main;
        let construct_rx = None;

        let admin = Admin::new();
        let action = Action::new();
        let world = World::new(template);
        let population = Population::new(template);
        let physics = Physics::new();
        let navigation = Navigation::new(&world.grid);
        let time = Time::new();

        Self {
            active,
            template,
            construct_rx,
            admin,
            action,
            time,
            physics,
            world,
            population,
            navigation,
        }
    }

    pub fn tick(state: &mut State) {
        let _ = tracing::info_span!("state_tick").entered();

        match state.admin.mode {
            admin::Mode::Menu => state_menu::tick(state),
            admin::Mode::Loading => state_loading::tick(state),
            admin::Mode::Simulate => state_simulate::tick(state),
            admin::Mode::Shutdown => state_shutdown::tick(state),
        }
    }

    pub fn place_block(block_kind: block::Kind, state: &mut State) {
        let origin = state.population.judge.spatial.world_position
            + Vec3::new(0.0, 0.0, state.population.judge.spatial.size.z);
        let dir = state.population.judge.spatial.rotor * Vec3::unit_y();

        if let Some((hit_position, normal)) =
            Self::raycast_first_solid(&state.world, origin, dir, 20.0)
        {
            let placement_position = hit_position + normal;

            World::set_block(
                placement_position,
                block_kind,
                &state.world.block_info_map,
                &state.world.grid,
                &mut state.world.sector_vec,
            );

            let block_info = state.world.block_info_map[&block_kind];

            Graph::set_solid(
                placement_position,
                block_info.solid,
                &mut state.navigation.graph,
            );
        }
    }

    pub fn remove_block(state: &mut State) {
        let origin = state.population.judge.spatial.world_position
            + Vec3::new(0.0, 0.0, state.population.judge.spatial.size.z);
        let dir = state.population.judge.spatial.rotor * Vec3::unit_y();

        if let Some((hit_position, _)) = Self::raycast_first_solid(&state.world, origin, dir, 20.0)
        {
            World::set_block(
                hit_position,
                block::Kind::None,
                &state.world.block_info_map,
                &state.world.grid,
                &mut state.world.sector_vec,
            );

            Graph::set_solid(hit_position, false, &mut state.navigation.graph);
        }
    }

    pub fn raycast_first_solid(
        world: &World,
        origin: Vec3,
        dir: Vec3,
        max_dist: f32,
    ) -> Option<(IVec3, IVec3)> {
        let dir = dir.normalized();

        // Start voxel
        let mut voxel = IVec3::new(
            origin.x.floor() as i32,
            origin.y.floor() as i32,
            origin.z.floor() as i32,
        );

        // Ray direction step for each axis
        let step = IVec3::new(
            if dir.x > 0.0 { 1 } else { -1 },
            if dir.y > 0.0 { 1 } else { -1 },
            if dir.z > 0.0 { 1 } else { -1 },
        );

        // Compute initial tMax (distance to first voxel boundary)
        let t_max = {
            let voxel_f = Vec3::new(voxel.x as f32, voxel.y as f32, voxel.z as f32);

            Vec3 {
                x: if dir.x != 0.0 {
                    ((voxel_f.x + (dir.x > 0.0) as u8 as f32) - origin.x) / dir.x
                } else {
                    f32::INFINITY
                },
                y: if dir.y != 0.0 {
                    ((voxel_f.y + (dir.y > 0.0) as u8 as f32) - origin.y) / dir.y
                } else {
                    f32::INFINITY
                },
                z: if dir.z != 0.0 {
                    ((voxel_f.z + (dir.z > 0.0) as u8 as f32) - origin.z) / dir.z
                } else {
                    f32::INFINITY
                },
            }
        };

        // How much t changes each time we step across a voxel boundary
        let t_delta = Vec3::new(
            if dir.x != 0.0 {
                (1.0 / dir.x).abs()
            } else {
                f32::INFINITY
            },
            if dir.y != 0.0 {
                (1.0 / dir.y).abs()
            } else {
                f32::INFINITY
            },
            if dir.z != 0.0 {
                (1.0 / dir.z).abs()
            } else {
                f32::INFINITY
            },
        );

        let mut t_max = t_max;
        let mut traveled = 0.0;

        // 3D DDA loop
        while traveled < max_dist {
            let hit_normal;

            // Step along the axis where tMax is smallest
            if t_max.x < t_max.y && t_max.x < t_max.z {
                voxel.x += step.x;
                hit_normal = IVec3::new(-step.x, 0, 0);
                traveled = t_max.x;
                t_max.x += t_delta.x;
            } else if t_max.y < t_max.z {
                voxel.y += step.y;
                hit_normal = IVec3::new(0, -step.y, 0);
                traveled = t_max.y;
                t_max.y += t_delta.y;
            } else {
                voxel.z += step.z;
                hit_normal = IVec3::new(0, 0, -step.z);
                traveled = t_max.z;
                t_max.z += t_delta.z;
            }

            let cell = World::get_cell_at(voxel, &world.grid, &world.sector_vec);

            if cell.solid {
                return Some((voxel, hit_normal));
            }
        }

        None
    }
}
