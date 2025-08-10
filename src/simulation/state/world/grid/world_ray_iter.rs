use crate::simulation::state::{
    physics::aabb::AABB,
    world::grid::{self, BlockSample, Grid},
    World,
};
use glam::{IVec3, Vec3};

pub struct WorldRayIter<'w> {
    world: &'w World,
    t: f32,
    origin: Vec3,
    direction: Vec3,
    max_t: f32,
    position: IVec3,
    step: IVec3,
    t_remaining: Vec3,
    t_delta: Vec3,
    done: bool,
}

impl<'w> WorldRayIter<'w> {
    /// Walk voxels intersected by origin + t * direction for t ∈ [0, distance], world-bounds clipped.
    pub fn new(world: &'w World, origin: Vec3, direction: Vec3, distance: f32) -> Option<Self> {
        if !direction.is_finite() || direction == Vec3::ZERO || distance <= 0.0 {
            return None;
        }

        let grid = &world.grid;
        let block_size = grid.block_size;
        let block_extent = grid.block_extent;

        let world_center = Vec3::ZERO;
        let world_aabb = AABB::new(world_center, Vec3::splat(grid.world_size_units));

        let (mut t0, mut t1) = ray_box_slab(origin, direction, &world_aabb);

        if !t0.is_finite() || !t1.is_finite() {
            return None;
        }

        t0 = t0.max(0.0);
        t1 = t1.min(distance);

        if t0 > t1 {
            return None;
        }

        let epsilon = 1e-4;
        let t_start = (t0 + epsilon).min(t1);
        let world_position = origin + direction * t_start;

        let position = Grid::world_to_position(&world.grid, world_position);

        let (sx, tdx, tx) = dda_axis_setup(
            position.x,
            world_position.x,
            direction.x,
            block_extent,
            block_size,
        );

        let (sy, tdy, ty) = dda_axis_setup(
            position.y,
            world_position.y,
            direction.y,
            block_extent,
            block_size,
        );

        let (sz, tdz, tz) = dda_axis_setup(
            position.z,
            world_position.z,
            direction.z,
            block_extent,
            block_size,
        );

        let step = IVec3::new(sx, sy, sz);
        let t_remaining = Vec3::new(tx, ty, tz);
        let t_delta = Vec3::new(tdx, tdy, tdz);

        Some(Self {
            world,
            t: t_start,
            origin,
            direction,
            max_t: t1,
            position,
            step,
            t_delta,
            t_remaining,
            done: false,
        })
    }
}

impl<'w> Iterator for WorldRayIter<'w> {
    type Item = BlockSample;

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

            let (axis, step_axis, tnext) = min3_axis(self.t_remaining, self.step);

            if !tnext.is_finite() || tnext > self.max_t {
                self.done = true;
                return None;
            }

            self.t = tnext;

            match axis {
                0 => {
                    self.t_remaining.x += self.t_delta.x;
                    self.position.x += step_axis;
                }
                1 => {
                    self.t_remaining.y += self.t_delta.y;
                    self.position.y += step_axis;
                }
                _ => {
                    self.t_remaining.z += self.t_delta.z;
                    self.position.z += step_axis;
                }
            }

            if !Grid::position_valid(grid, self.position) {
                self.done = true;
                return None;
            }

            let enter_face_direction = face_from_axis_step(axis, step_axis);

            let (chunk_id, block_id) = Grid::position_to_ids(grid, self.position);

            let world_position = self.origin + self.direction * self.t;

            return Some(BlockSample {
                t: self.t,
                position: self.position,
                world_position,
                chunk_id,
                block_id,
                enter_face_direction,
            });
        }
    }
}

/* ================= helpers (private) ================= */

#[inline]
fn ray_box_slab(origin: Vec3, direction: Vec3, slab_aabb: &AABB) -> (f32, f32) {
    let (tx0, tx1) = axis_slab(origin.x, direction.x, slab_aabb.min.x, slab_aabb.max.x);
    let (ty0, ty1) = axis_slab(origin.y, direction.y, slab_aabb.min.y, slab_aabb.max.y);
    let (tz0, tz1) = axis_slab(origin.z, direction.z, slab_aabb.min.z, slab_aabb.max.z);

    let t_enter = tx0.max(ty0).max(tz0);
    let t_exit = tx1.min(ty1).min(tz1);

    if !t_enter.is_finite() || !t_exit.is_finite() || t_enter > t_exit {
        return (f32::NAN, f32::NAN);
    }

    (t_enter, t_exit)
}

#[inline]
fn axis_slab(o: f32, d: f32, mn: f32, mx: f32) -> (f32, f32) {
    if d == 0.0 {
        if o < mn || o > mx {
            return (f32::NAN, f32::NAN);
        } else {
            return (f32::NEG_INFINITY, f32::INFINITY);
        }
    }

    let inv = 1.0 / d;
    let mut t0 = (mn - o) * inv;
    let mut t1 = (mx - o) * inv;

    if t0 > t1 {
        core::mem::swap(&mut t0, &mut t1);
    }

    (t0, t1)
}

#[inline]
fn dda_axis_setup(
    axis_position: i32,
    axis_world_position: f32,
    axis_direction: f32,
    block_extent: f32,
    block_size: f32,
) -> (i32, f32, f32) {
    use core::f32::INFINITY;

    let step = if axis_direction > 0.0 {
        1
    } else if axis_direction < 0.0 {
        -1
    } else {
        return (0, INFINITY, INFINITY);
    };

    let next_face_world_position = (axis_position as f32)
        + if step > 0 {
            block_extent
        } else {
            -block_extent
        };

    let t_max = (next_face_world_position - axis_world_position) / axis_direction;
    let t_delta = block_size / axis_direction.abs();

    (step, t_delta, t_max)
}

#[inline]
fn min3_axis(t_remaining: Vec3, step: IVec3) -> (u8, i32, f32) {
    let txp = if step.x == 0 || !t_remaining.x.is_finite() {
        f32::INFINITY
    } else {
        t_remaining.x
    };

    let typ = if step.y == 0 || !t_remaining.y.is_finite() {
        f32::INFINITY
    } else {
        t_remaining.y
    };

    let tzp = if step.z == 0 || !t_remaining.z.is_finite() {
        f32::INFINITY
    } else {
        t_remaining.z
    };

    let mut axis: u8 = 0;
    let mut step_axis = step.x;
    let mut tnext = txp;

    if typ < tnext {
        axis = 1;
        step_axis = step.y;
        tnext = typ;
    }

    if tzp < tnext {
        axis = 2;
        step_axis = step.z;
        tnext = tzp;
    }

    (axis, step_axis, tnext)
}

#[inline]
fn face_from_axis_step(axis: u8, step: i32) -> grid::Direction {
    match (axis, step) {
        (0, 1) => grid::Direction::XnYoZo,
        (0, -1) => grid::Direction::XpYoZo,
        (1, 1) => grid::Direction::XoYnZo,
        (1, -1) => grid::Direction::XoYpZo,
        (2, 1) => grid::Direction::XoYoZn,
        (2, -1) => grid::Direction::XoYoZp,
        _ => panic!("Requesting face for non-axis direction"),
    }
}
