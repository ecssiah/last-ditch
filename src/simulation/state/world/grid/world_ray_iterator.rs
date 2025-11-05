use crate::simulation::state::{
    physics::aabb::AABB,
    world::grid::{self, CellSample, Grid},
    World,
};
use glam::{IVec3, Vec3};

pub struct WorldRayIterator<'world> {
    world: &'world World,
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

impl<'world> WorldRayIterator<'world> {
    pub fn from_ray(
        world: &'world World,
        origin: Vec3,
        direction: Vec3,
        distance: f32,
    ) -> Option<Self> {
        if !direction.is_finite() || direction == Vec3::ZERO || distance <= 0.0 {
            return None;
        }

        let grid = &world.grid;
        let cell_size_in_meters = grid.cell_size_in_meters;
        let cell_radius_in_meters = grid.cell_radius_in_meters;

        let world_aabb = AABB::new(Vec3::ZERO, Vec3::splat(grid.world_size_in_meters));

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
        let world_position = origin + direction * t0 - direction.signum() * epsilon;

        let position = Grid::world_to_position(&world.grid, world_position);

        let (step_direction_x, t_delta_x, t_remaining_x) = dda_axis_setup(
            position.x,
            world_position.x,
            direction.x,
            cell_radius_in_meters,
            cell_size_in_meters,
        );

        let (step_direction_y, t_delta_y, t_remaining_y) = dda_axis_setup(
            position.y,
            world_position.y,
            direction.y,
            cell_radius_in_meters,
            cell_size_in_meters,
        );

        let (step_direction_z, t_delta_z, t_remaining_z) = dda_axis_setup(
            position.z,
            world_position.z,
            direction.z,
            cell_radius_in_meters,
            cell_size_in_meters,
        );

        let step_direction = IVec3::new(step_direction_x, step_direction_y, step_direction_z);
        let t_delta = Vec3::new(t_delta_x, t_delta_y, t_delta_z);
        let t_remaining = Vec3::new(t_remaining_x, t_remaining_y, t_remaining_z);

        let world_ray_iterator = Self {
            world,
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

impl<'w> Iterator for WorldRayIterator<'w> {
    type Item = CellSample;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let grid = &self.world.grid;

        loop {
            if !self.t_remaining.x.is_finite()
                && !self.t_remaining.y.is_finite()
                && !self.t_remaining.z.is_finite()
            {
                self.done = true;
                return None;
            }

            let (axis, step_direction_axis, t_next) =
                get_next_cell_boundary_info(self.step_direction, self.t_remaining);

            if !t_next.is_finite() || t_next > self.t_max {
                self.done = true;
                return None;
            }

            self.t = t_next;

            match axis {
                grid::Axis::X => {
                    self.t_remaining.x += self.t_delta.x;
                    self.position.x += step_direction_axis;
                }
                grid::Axis::Y => {
                    self.t_remaining.y += self.t_delta.y;
                    self.position.y += step_direction_axis;
                }
                grid::Axis::Z => {
                    self.t_remaining.z += self.t_delta.z;
                    self.position.z += step_direction_axis;
                }
            }

            if !Grid::position_valid(grid, self.position) {
                self.done = true;
                return None;
            }

            let enter_face_direction = get_face_direction(axis, step_direction_axis);

            let (sector_id, cell_id) = Grid::position_to_ids(grid, self.position);

            let world_position = self.origin + self.direction * self.t;

            let cell_sample = CellSample {
                t: self.t,
                position: self.position,
                world_position,
                sector_id,
                cell_id,
                enter_face_direction,
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
    cell_radius_in_meters: f32,
    cell_size_in_meters: f32,
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
            cell_radius_in_meters
        } else {
            -cell_radius_in_meters
        };

    let t_delta = cell_size_in_meters / direction_axis.abs();
    let t_max = (next_face_world_position - world_position_axis) / direction_axis;

    (step_direction, t_delta, t_max)
}

#[inline]
fn get_next_cell_boundary_info(step_direction: IVec3, t_remaining: Vec3) -> (grid::Axis, i32, f32) {
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

    let mut axis = grid::Axis::X;
    let mut step_direction_axis = step_direction.x;
    let mut t_next = txp;

    if typ < t_next {
        axis = grid::Axis::Y;
        step_direction_axis = step_direction.y;
        t_next = typ;
    }

    if tzp < t_next {
        axis = grid::Axis::Z;
        step_direction_axis = step_direction.z;
        t_next = tzp;
    }

    (axis, step_direction_axis, t_next)
}

#[inline]
fn get_face_direction(axis: grid::Axis, step_direction: i32) -> grid::Direction {
    match (axis, step_direction) {
        (grid::Axis::X, 1) => grid::Direction::XnYoZo,
        (grid::Axis::X, -1) => grid::Direction::XpYoZo,
        (grid::Axis::Y, 1) => grid::Direction::XoYnZo,
        (grid::Axis::Y, -1) => grid::Direction::XoYpZo,
        (grid::Axis::Z, 1) => grid::Direction::XoYoZn,
        (grid::Axis::Z, -1) => grid::Direction::XoYoZp,
        _ => panic!("Requesting face for non-axis direction"),
    }
}
