use crate::simulation::{
    consts::GOLDEN_ANGLE,
    state::{
        population::entity::{self, Spatial},
        world::{chunk, grid::WorldRayIter},
        World,
    },
};
use glam::Vec3;
use std::collections::HashSet;

pub struct Sight {
    pub distance: f32,
    pub fov_angle: f32,
    pub direction_vec: Vec<Vec3>,
    pub chunk_id_set: HashSet<chunk::ID>,
    pub entity_id_set: HashSet<entity::ID>,
}

impl Sight {
    pub fn new(distance: f32, fov_angle: f32) -> Self {
        let direction_vec = Self::generate_fibonacci_cone_direction_vec(90.0_f32.to_radians(), 100);

        let chunk_id_set = HashSet::new();
        let entity_id_set = HashSet::new();

        Self {
            distance,
            fov_angle,
            direction_vec,
            chunk_id_set,
            entity_id_set,
        }
    }

    pub fn tick(world: &World, spatial: &Spatial, sight: &mut Sight) {
        let broadphase_chunk_id_set = Self::broadphase_filter(world, spatial, sight);

        let narrowphase_chunk_id_set =
            Self::narrowphase_filter(&broadphase_chunk_id_set, world, spatial, sight);

        sight.chunk_id_set = narrowphase_chunk_id_set;

        println!("{:?}", sight.chunk_id_set.iter().len());
    }

    fn broadphase_filter(world: &World, spatial: &Spatial, sight: &Sight) -> HashSet<chunk::ID> {
        let half_fov_angle = sight.fov_angle / 2.0;
        let (sin_a, cos_a) = half_fov_angle.sin_cos();
        let chunk_sphere_radius =
            world.grid.chunk_extent_blocks as f32 * world.grid.block_extent * 3.0_f32.sqrt();

        let mut chunk_id_set = HashSet::new();

        for chunk in &world.chunk_vec {
            let eye_to_center = chunk.position.as_vec3() - spatial.eye();
            let eye_to_center_length_squared = eye_to_center.length_squared();

            if eye_to_center_length_squared == 0.0 {
                chunk_id_set.insert(chunk.id);
                continue;
            }

            let eye_to_center_length = eye_to_center.length();

            if eye_to_center_length > sight.distance + chunk_sphere_radius {
                continue;
            }

            if eye_to_center_length <= chunk_sphere_radius {
                chunk_id_set.insert(chunk.id);
                continue;
            }

            let angular_pad = (chunk_sphere_radius / eye_to_center_length).min(1.0);
            let cos_b = (1.0 - angular_pad * angular_pad).max(0.0).sqrt();
            let rhs = cos_a * cos_b - sin_a * angular_pad;

            let dot = spatial.forward().dot(eye_to_center / eye_to_center_length);

            if dot >= rhs {
                chunk_id_set.insert(chunk.id);
            }
        }

        chunk_id_set
    }

    fn narrowphase_filter(
        broadphase_chunk_id_set: &HashSet<chunk::ID>,
        world: &World,
        spatial: &Spatial,
        sight: &Sight,
    ) -> HashSet<chunk::ID> {
        let mut chunk_id_set = HashSet::new();

        let rotated_cone_direction_vec =
            Sight::rotated_direction_vec(spatial.forward(), &sight.direction_vec);

        for direction_vec in rotated_cone_direction_vec {
            let t_epsilon = world.grid.block_extent * 0.01;
            let ray_origin = spatial.eye() + direction_vec * t_epsilon;

            if let Some(mut world_ray_iter) =
                WorldRayIter::from_ray(world, ray_origin, direction_vec, sight.distance)
            {
                while let Some(block_sample) = world_ray_iter.next() {
                    if broadphase_chunk_id_set.contains(&block_sample.chunk_id) {
                        if let Some(block) = World::get_block(
                            block_sample.chunk_id,
                            block_sample.block_id,
                            &world.chunk_vec,
                        ) {
                            if block.solid {
                                chunk_id_set.insert(block_sample.chunk_id);
                                break;
                            }
                        }
                    }
                }
            }
        }

        chunk_id_set
    }

    fn compute_rotation_matrix(forward: Vec3) -> glam::Mat3 {
        let from = Vec3::Z;
        let to = forward.normalize();

        let from_cross_to = from.cross(to);
        let from_dot_to = from.dot(to);

        if from_cross_to.length_squared() < 1e-6 {
            if from_dot_to > 0.0 {
                glam::Mat3::IDENTITY
            } else {
                glam::Mat3::from_axis_angle(Vec3::X, std::f32::consts::PI)
            }
        } else {
            let from_cross_to_magnitude = from_cross_to.length();

            let k_matrix = glam::Mat3::from_cols(
                Vec3::new(0.0, -from_cross_to.z, from_cross_to.y),
                Vec3::new(from_cross_to.z, 0.0, -from_cross_to.x),
                Vec3::new(-from_cross_to.y, from_cross_to.x, 0.0),
            );

            glam::Mat3::IDENTITY
                + k_matrix
                + (k_matrix * k_matrix)
                    * ((1.0 - from_dot_to) / (from_cross_to_magnitude * from_cross_to_magnitude))
        }
    }

    pub fn rotated_direction_vec(forward: Vec3, direction_vec: &Vec<Vec3>) -> Vec<Vec3> {
        let rotation_matrix = Self::compute_rotation_matrix(forward);

        direction_vec
            .iter()
            .map(|direction| rotation_matrix * *direction)
            .collect()
    }

    pub fn generate_fibonacci_cone_direction_vec(angle: f32, ray_count: usize) -> Vec<Vec3> {
        let mut direction_vec = Vec::with_capacity(ray_count);
        let angle_cos = angle.cos();

        for i in 0..ray_count {
            let i = i as f32;
            let count = ray_count as f32;

            let z = angle_cos + (1.0 - angle_cos) * ((i + 0.5) / count);
            let radius = (1.0 - z * z).sqrt();
            let phi = GOLDEN_ANGLE * i;

            let x = radius * phi.cos();
            let y = radius * phi.sin();

            direction_vec.push(Vec3::new(x, y, z));
        }

        direction_vec
    }
}
