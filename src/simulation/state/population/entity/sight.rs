use crate::simulation::{
    consts::GOLDEN_ANGLE,
    state::{
        population::entity::{self, Spatial},
        world::{chunk, grid::WorldRayIterator},
        World,
    },
};
use glam::Vec3;
use std::collections::HashSet;

pub struct Sight {
    pub distance: f32,
    pub fov_angle: f32,
    pub view_ray_vec_reference: Vec<Vec3>,
    pub view_ray_vec: Vec<Vec3>,
    pub chunk_id_set: HashSet<chunk::ID>,
    pub entity_id_set: HashSet<entity::ID>,
}

impl Sight {
    pub fn new(fov_angle: f32, distance: f32) -> Self {
        let view_ray_vec_reference = Self::generate_view_ray_vec_reference(fov_angle, 320);
        let view_ray_vec = Vec::new();

        let chunk_id_set = HashSet::new();
        let entity_id_set = HashSet::new();

        Self {
            distance,
            fov_angle,
            view_ray_vec_reference,
            view_ray_vec,
            chunk_id_set,
            entity_id_set,
        }
    }

    pub fn tick(world: &World, spatial: &Spatial, sight: &mut Sight) {
        let broadphase_chunk_id_set = Self::broadphase_filter(world, spatial, sight);

        sight.view_ray_vec =
            Sight::rotate_view_ray_vec(spatial.forward(), &sight.view_ray_vec_reference);

        let narrowphase_chunk_id_set =
            Self::narrowphase_filter(&broadphase_chunk_id_set, world, spatial, sight);

        sight.chunk_id_set = narrowphase_chunk_id_set;
    }

    fn broadphase_filter(world: &World, spatial: &Spatial, sight: &Sight) -> HashSet<chunk::ID> {
        let fov_angle_radians = sight.fov_angle.to_radians();

        let half_fov_angle = fov_angle_radians / 2.0;
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

            let near_ring_multiplier = 2.5_f32;
            let near_distance = chunk_sphere_radius * near_ring_multiplier;
            
            if eye_to_center_length <= near_distance {
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

        for &ray in &sight.view_ray_vec {
            let t_epsilon = world.grid.block_extent * 0.01;
            let ray_origin = spatial.eye() + ray * t_epsilon;

            if let Some(mut world_ray_iterator) =
                WorldRayIterator::from_ray(world, ray_origin, ray, sight.distance)
            {
                while let Some(block_sample) = world_ray_iterator.next() {
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

    fn basis_from_forward(forward: Vec3, up_hint: Vec3) -> glam::Mat3 {
        let f = forward.normalize();
        if f.length_squared() == 0.0 {
            return glam::Mat3::IDENTITY;
        }

        let mut r = up_hint.cross(f);

        if r.length_squared() < 1e-6 {
            let alt = if f.y.abs() < 0.9 { Vec3::Y } else { Vec3::X };
            r = alt.cross(f);
        }
        r = r.normalize();

        let u = f.cross(r);

        glam::Mat3::from_cols(r, u, f)
    }

    pub fn rotate_view_ray_vec(forward: Vec3, view_ray_vec: &Vec<Vec3>) -> Vec<Vec3> {
        let basis = Self::basis_from_forward(forward, Vec3::Y);

        view_ray_vec.iter().map(|ray| basis * *ray).collect()
    }

    pub fn generate_view_ray_vec_reference(fov_angle: f32, ray_count: usize) -> Vec<Vec3> {
        debug_assert!(ray_count > 0, "view ray count is zero");

        let fov_angle_radians = fov_angle.to_radians();

        let ray_count_f32 = ray_count as f32;
        let mut ray_vec = Vec::with_capacity(ray_count);

        let half_fov_angle = fov_angle_radians * 0.5;
        let cos_half_fov_angle = half_fov_angle.cos();

        let dz = (1.0 - cos_half_fov_angle) / ray_count_f32;
        let mut z = cos_half_fov_angle + dz * 0.5;

        for ray_index in 0..ray_count {
            let phi = GOLDEN_ANGLE * ray_index as f32;

            let z_clamped = z.clamp(-1.0, 1.0);
            let radius = (1.0 - z_clamped * z_clamped).sqrt();

            let (phi_sin, phi_cos) = phi.sin_cos();

            let x = radius * phi_cos;
            let y = radius * phi_sin;

            let ray = Vec3::new(x, y, z_clamped).normalize();

            ray_vec.push(ray);

            z += dz;
        }

        ray_vec
    }
}
