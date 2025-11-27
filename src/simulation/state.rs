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
pub use world::World;

use crate::simulation::state::{
    self,
    navigation::{Graph, Navigation},
    world::{block, grid::Grid},
};
use ultraviolet::{IVec3, Vec3};

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
        let range = 8.0;
        let origin = state.population.judge.sight.world_position;
        let direction = state.population.judge.sight.rotor * Vec3::unit_y();

        tracing::info!("Origin: {:?}", origin);
        tracing::info!("Direction: {:?}", direction);

        if let Some((hit_position, normal)) =
            Self::raycast_to_block(&state.world, origin, direction, range)
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
        let range = 8.0;
        let origin = state.population.judge.sight.world_position;
        let direction = state.population.judge.sight.rotor * Vec3::unit_y();

        if let Some((hit_position, _)) =
            Self::raycast_to_block(&state.world, origin, direction, range)
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

    pub fn raycast_to_block(
        world: &World,
        origin: Vec3,
        direction: Vec3,
        range: f32,
    ) -> Option<(IVec3, IVec3)> {
        let direction = direction.normalized();

        let mut cell_position = Grid::world_to_cell_coordinates(origin, &world.grid);

        let step = IVec3::new(
            if direction.x > 0.0 { 1 } else { -1 },
            if direction.y > 0.0 { 1 } else { -1 },
            if direction.z > 0.0 { 1 } else { -1 },
        );

        let cell_world_position_min = Vec3::new(
            cell_position.x as f32 - world.grid.cell_radius_in_meters,
            cell_position.y as f32 - world.grid.cell_radius_in_meters,
            cell_position.z as f32 - world.grid.cell_radius_in_meters,
        );

        let t_max = Vec3 {
            x: if direction.x != 0.0 {
                let boundary = if direction.x > 0.0 {
                    cell_world_position_min.x + world.grid.cell_size_in_meters
                } else {
                    cell_world_position_min.x
                };
                (boundary - origin.x) / direction.x
            } else {
                f32::INFINITY
            },
            y: if direction.y != 0.0 {
                let boundary = if direction.y > 0.0 {
                    cell_world_position_min.y + world.grid.cell_size_in_meters
                } else {
                    cell_world_position_min.y
                };
                (boundary - origin.y) / direction.y
            } else {
                f32::INFINITY
            },
            z: if direction.z != 0.0 {
                let boundary = if direction.z > 0.0 {
                    cell_world_position_min.z + world.grid.cell_size_in_meters
                } else {
                    cell_world_position_min.z
                };
                (boundary - origin.z) / direction.z
            } else {
                f32::INFINITY
            },
        };

        let t_delta = Vec3::new(
            if direction.x != 0.0 {
                (1.0 / direction.x).abs()
            } else {
                f32::INFINITY
            },
            if direction.y != 0.0 {
                (1.0 / direction.y).abs()
            } else {
                f32::INFINITY
            },
            if direction.z != 0.0 {
                (1.0 / direction.z).abs()
            } else {
                f32::INFINITY
            },
        );

        let mut t_max = t_max;
        let mut distance_traveled = 0.0;

        // 3D DDA loop
        while distance_traveled < range {
            let hit_normal;

            if t_max.x < t_max.y && t_max.x < t_max.z {
                cell_position.x += step.x;
                distance_traveled = t_max.x;
                t_max.x += t_delta.x;

                hit_normal = -step.x * IVec3::unit_x();
            } else if t_max.y < t_max.z {
                cell_position.y += step.y;
                distance_traveled = t_max.y;
                t_max.y += t_delta.y;

                hit_normal = -step.y * IVec3::unit_y();
            } else {
                cell_position.z += step.z;
                distance_traveled = t_max.z;
                t_max.z += t_delta.z;

                hit_normal = -step.z * IVec3::unit_z();
            }

            let cell = World::get_cell_at(cell_position, &world.grid, &world.sector_vec);

            if cell.solid {
                return Some((cell_position, hit_normal));
            }
        }

        None
    }
}
