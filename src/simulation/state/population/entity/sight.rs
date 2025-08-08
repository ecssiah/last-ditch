use crate::simulation::{
    consts::GOLDEN_ANGLE,
    state::{
        population::entity::{self, Spatial},
        world::chunk,
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
        let rotated_cone_directions = sight.rotated_direction_vec(spatial.forward());

        // ray trace
    }

    fn compute_rotation_matrix(forward: Vec3) -> glam::Mat3 {
        let from = Vec3::Z;
        let to = forward.normalize();

        let v = from.cross(to);
        let c = from.dot(to);

        if v.length_squared() < 1e-6 {
            if c > 0.0 {
                glam::Mat3::IDENTITY
            } else {
                glam::Mat3::from_axis_angle(Vec3::X, std::f32::consts::PI)
            }
        } else {
            let s = v.length();

            let kmat = glam::Mat3::from_cols(
                Vec3::new(0.0, -v.z, v.y),
                Vec3::new(v.z, 0.0, -v.x),
                Vec3::new(-v.y, v.x, 0.0),
            );

            glam::Mat3::IDENTITY + kmat + (kmat * kmat) * ((1.0 - c) / (s * s))
        }
    }

    pub fn rotated_direction_vec(&self, forward: Vec3) -> Vec<Vec3> {
        let rotation_matrix = Self::compute_rotation_matrix(forward);

        self.direction_vec
            .iter()
            .map(|direction| rotation_matrix * *direction)
            .collect()
    }

    pub fn generate_fibonacci_cone_direction_vec(angle: f32, ray_count: usize) -> Vec<Vec3> {
        let mut direction_vec = Vec::with_capacity(ray_count);
        let angle_cos = angle.cos();
        let golden_angle = GOLDEN_ANGLE;

        for i in 0..ray_count {
            let i = i as f32;
            let count = ray_count as f32;

            let z = angle_cos + (1.0 - angle_cos) * ((i + 0.5) / count);
            let radius = (1.0 - z * z).sqrt();
            let phi = golden_angle * i;

            let x = radius * phi.cos();
            let y = radius * phi.sin();

            direction_vec.push(Vec3::new(x, y, z));
        }

        direction_vec
    }
}
