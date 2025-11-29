use crate::simulation::{
    constants::{CELL_RADIUS_IN_METERS, CELL_SIZE_IN_METERS, WORLD_SIZE_IN_METERS},
    state::{
        physics::aabb::AABB,
        world::grid::{self, CellSample},
    },
};
use ultraviolet::{IVec3, Vec3};

pub struct WorldRayIterator {
    done: bool,
    t: f32,
    origin: Vec3,
    direction: Vec3,
    t_max: f32,
    position: IVec3,
    step_direction: IVec3,
    t_remaining: Vec3,
    t_delta: Vec3,
}

impl WorldRayIterator {
    pub fn from_ray(origin: Vec3, direction: Vec3, distance: f32) -> Option<Self> {
        if distance <= 0.0 {
            return None;
        }

        if direction == Vec3::broadcast(0.0) {
            return None;
        }

        if !(direction.x.is_finite() && direction.y.is_finite() && direction.z.is_finite()) {
            return None;
        }

        let world_aabb = AABB::new(Vec3::broadcast(0.0), Vec3::broadcast(WORLD_SIZE_IN_METERS));

        let (mut t0, mut t1) = slab_test(origin, direction, &world_aabb);

        if !t0.is_finite() || !t1.is_finite() {
            return None;
        }

        t0 = t0.max(0.0);
        t1 = t1.min(distance);

        if t0 > t1 {
            return None;
        }

        let t_start = t0;
        let epsilon = 1e-6;

        let direction_signum = Vec3::new(
            direction.x.signum(),
            direction.y.signum(),
            direction.z.signum(),
        );

        let world_position = origin + direction * t0 - direction_signum * epsilon;
        let position = grid::world_position_to_position(world_position);

        let (step_direction_x, t_delta_x, t_remaining_x) =
            dda_axis_setup(position.x, world_position.x, direction.x);

        let (step_direction_y, t_delta_y, t_remaining_y) =
            dda_axis_setup(position.y, world_position.y, direction.y);

        let (step_direction_z, t_delta_z, t_remaining_z) =
            dda_axis_setup(position.z, world_position.z, direction.z);

        let step_direction = IVec3::new(step_direction_x, step_direction_y, step_direction_z);
        let t_delta = Vec3::new(t_delta_x, t_delta_y, t_delta_z);
        let t_remaining = Vec3::new(t_remaining_x, t_remaining_y, t_remaining_z);

        let world_ray_iterator = Self {
            done: false,
            t: t_start,
            origin,
            direction,
            t_max: t1,
            position,
            step_direction,
            t_delta,
            t_remaining,
        };

        Some(world_ray_iterator)
    }
}

impl Iterator for WorldRayIterator {
    type Item = CellSample;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        loop {
            if !self.t_remaining.x.is_finite()
                && !self.t_remaining.y.is_finite()
                && !self.t_remaining.z.is_finite()
            {
                self.done = true;
                return None;
            }

            let (direction_entered, t_next) =
                get_next_cell_boundary_info(self.step_direction, self.t_remaining);

            if !t_next.is_finite() || t_next > self.t_max {
                self.done = true;
                return None;
            }

            self.t = t_next;
            self.position += direction_entered.to_ivec3();

            match direction_entered {
                grid::Direction::East | grid::Direction::West => {
                    self.t_remaining.x += self.t_delta.x;
                }
                grid::Direction::North | grid::Direction::South => {
                    self.t_remaining.y += self.t_delta.y;
                }
                grid::Direction::Up | grid::Direction::Down => {
                    self.t_remaining.z += self.t_delta.z;
                }
            }

            if !grid::position_valid(self.position) {
                self.done = true;

                return None;
            }

            let (sector_id, cell_id) = grid::position_to_ids(self.position);

            let world_position = self.origin + self.direction * self.t;

            let cell_sample = CellSample {
                t: self.t,
                position: self.position,
                world_position,
                sector_id,
                cell_id,
                direction_entered,
            };

            return Some(cell_sample);
        }
    }
}

/* ================= helpers (private) ================= */

#[inline]
fn slab_test(origin: Vec3, direction: Vec3, aabb: &AABB) -> (f32, f32) {
    let (tx0, tx1) = slab_test_axis(origin.x, direction.x, aabb.min.x, aabb.max.x);
    let (ty0, ty1) = slab_test_axis(origin.y, direction.y, aabb.min.y, aabb.max.y);
    let (tz0, tz1) = slab_test_axis(origin.z, direction.z, aabb.min.z, aabb.max.z);

    let t_enter = tx0.max(ty0).max(tz0);
    let t_exit = tx1.min(ty1).min(tz1);

    if !t_enter.is_finite() || !t_exit.is_finite() || t_enter > t_exit {
        return (f32::NAN, f32::NAN);
    }

    (t_enter, t_exit)
}

#[inline]
fn slab_test_axis(
    origin_axis: f32,
    direction_axis: f32,
    min_face_axis: f32,
    max_face_axis: f32,
) -> (f32, f32) {
    if direction_axis == 0.0 {
        if origin_axis < min_face_axis || origin_axis > max_face_axis {
            return (f32::NAN, f32::NAN);
        } else {
            return (f32::NEG_INFINITY, f32::INFINITY);
        }
    }

    let direction_axis_inverse = direction_axis.recip();

    let mut t0 = (min_face_axis - origin_axis) * direction_axis_inverse;
    let mut t1 = (max_face_axis - origin_axis) * direction_axis_inverse;

    if t0 > t1 {
        core::mem::swap(&mut t0, &mut t1);
    }

    (t0, t1)
}

#[inline]
fn dda_axis_setup(
    position_axis: i32,
    world_position_axis: f32,
    direction_axis: f32,
) -> (i32, f32, f32) {
    use core::f32::INFINITY;

    let step_direction = if direction_axis > 0.0 {
        1
    } else if direction_axis < 0.0 {
        -1
    } else {
        return (0, INFINITY, INFINITY);
    };

    let next_face_world_position = (position_axis as f32)
        + if step_direction > 0 {
            CELL_RADIUS_IN_METERS
        } else {
            -CELL_RADIUS_IN_METERS
        };

    let t_delta = CELL_SIZE_IN_METERS / direction_axis.abs();
    let t_max = (next_face_world_position - world_position_axis) / direction_axis;

    (step_direction, t_delta, t_max)
}

#[inline]
fn get_next_cell_boundary_info(step_direction: IVec3, t_remaining: Vec3) -> (grid::Direction, f32) {
    let txp = if step_direction.x == 0 || !t_remaining.x.is_finite() {
        f32::INFINITY
    } else {
        t_remaining.x
    };

    let typ = if step_direction.y == 0 || !t_remaining.y.is_finite() {
        f32::INFINITY
    } else {
        t_remaining.y
    };

    let tzp = if step_direction.z == 0 || !t_remaining.z.is_finite() {
        f32::INFINITY
    } else {
        t_remaining.z
    };

    let mut axis = if step_direction.x > 0 {
        grid::Direction::East
    } else {
        grid::Direction::West
    };

    let mut t_next = txp;

    if typ < t_next {
        axis = if step_direction.y > 0 {
            grid::Direction::North
        } else {
            grid::Direction::South
        };

        t_next = typ;
    }

    if tzp < t_next {
        axis = if step_direction.z > 0 {
            grid::Direction::Up
        } else {
            grid::Direction::Down
        };

        t_next = tzp;
    }

    (axis, t_next)
}
