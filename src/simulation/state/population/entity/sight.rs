use crate::simulation::{
    consts::GOLDEN_ANGLE,
    state::{
        physics::aabb::AABB,
        population::entity::{self, Spatial},
        world::{
            chunk,
            grid::{self},
        },
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

        let rotated_cone_direction_vec =
            Sight::rotated_direction_vec(spatial.forward(), &sight.direction_vec);

        println!("Broad: {:?}", broadphase_chunk_id_set.len());

        let narrowphase_chunk_id_set = Self::narrowphase_filter(
            &broadphase_chunk_id_set,
            rotated_cone_direction_vec,
            world,
            spatial,
            sight,
        );

        println!("Narrow: {:?}", narrowphase_chunk_id_set.len());
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
        chunk_id_vec: &HashSet<chunk::ID>,
        cone_direction_vec: Vec<Vec3>,
        world: &World,
        spatial: &Spatial,
        sight: &Sight,
    ) -> HashSet<chunk::ID> {
        HashSet::new()
    }

    #[inline]
    fn face_direction_from_axis_step(axis: u8, step: i32) -> grid::Direction {
        match (axis, step) {
            (0, 1) => grid::Direction::XnYoZo,
            (0, -1) => grid::Direction::XpYoZo,
            (1, 1) => grid::Direction::XoYnZo,
            (1, -1) => grid::Direction::XoYpZo,
            (2, 1) => grid::Direction::XoYoZn,
            (2, -1) => grid::Direction::XoYoZp,
            _ => unreachable!(),
        }
    }

    #[inline]
    fn min3_axis(tx: f32, ty: f32, tz: f32, sx: i32, sy: i32, sz: i32) -> (u8, i32, f32) {
        let txp = if sx == 0 || !tx.is_finite() {
            f32::INFINITY
        } else {
            tx
        };
        let typ = if sy == 0 || !ty.is_finite() {
            f32::INFINITY
        } else {
            ty
        };
        let tzp = if sz == 0 || !tz.is_finite() {
            f32::INFINITY
        } else {
            tz
        };

        let mut axis: u8 = 0;
        let mut step = sx;
        let mut tnext = txp;

        if typ < tnext {
            axis = 1;
            step = sy;
            tnext = typ;
        }
        if tzp < tnext {
            axis = 2;
            step = sz;
            tnext = tzp;
        }

        (axis, step, tnext)
    }

    #[inline]
    fn dda_axis_setup(pos_axis: f32, dir_axis: f32, block_size: f32) -> (i32, f32, f32) {
        use std::f32::INFINITY;

        let step = if dir_axis > 0.0 {
            1
        } else if dir_axis < 0.0 {
            -1
        } else {
            return (0, INFINITY, INFINITY);
        };

        let r = 0.5 * block_size;

        let next_face = if step > 0 {
            (pos_axis - r).ceil() + r
        } else {
            (pos_axis + r).floor() - r
        };

        let t_max = (next_face - pos_axis) / dir_axis;
        let t_delta = block_size / dir_axis.abs();

        (step, t_delta, t_max)
    }

    #[inline]
    fn ray_box_slab(origin: Vec3, dir: Vec3, aabb: AABB) -> (f32, f32) {
        let (tx_min, tx_max) = if dir.x == 0.0 {
            if origin.x < aabb.min.x || origin.x > aabb.max.x {
                return (f32::NAN, f32::NAN);
            }

            (f32::NEG_INFINITY, f32::INFINITY)
        } else {
            let inv = 1.0 / dir.x;
            let mut t1 = (aabb.min.x - origin.x) * inv;
            let mut t2 = (aabb.max.x - origin.x) * inv;

            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }

            (t1, t2)
        };

        let (ty_min, ty_max) = if dir.y == 0.0 {
            if origin.y < aabb.min.y || origin.y > aabb.max.y {
                return (f32::NAN, f32::NAN);
            }

            (f32::NEG_INFINITY, f32::INFINITY)
        } else {
            let inv = 1.0 / dir.y;
            let mut t1 = (aabb.min.y - origin.y) * inv;
            let mut t2 = (aabb.max.y - origin.y) * inv;

            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }

            (t1, t2)
        };

        let (tz_min, tz_max) = if dir.z == 0.0 {
            if origin.z < aabb.min.z || origin.z > aabb.max.z {
                return (f32::NAN, f32::NAN);
            }

            (f32::NEG_INFINITY, f32::INFINITY)
        } else {
            let inv = 1.0 / dir.z;
            let mut t1 = (aabb.min.z - origin.z) * inv;
            let mut t2 = (aabb.max.z - origin.z) * inv;

            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }

            (t1, t2)
        };

        let t_enter = tx_min.max(ty_min).max(tz_min);
        let t_exit = tx_max.min(ty_max).min(tz_max);

        if t_enter > t_exit {
            (f32::NAN, f32::NAN)
        } else {
            (t_enter, t_exit)
        }
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
