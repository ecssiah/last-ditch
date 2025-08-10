use crate::simulation::state::{
    physics::aabb::AABB,
    world::grid::{self, VoxelSample},
    World,
};
use glam::{IVec3, Vec3};

pub struct WorldRayIter<'w> {
    world: &'w World,
    t: f32,
    origin: Vec3,
    direction: Vec3,
    max_t: f32,
    voxel_indices: IVec3,
    step: IVec3,
    t_remaining: Vec3,
    t_delta: Vec3,
    block_size: f32,
    block_extent: f32,
    world_aabb: AABB,
    done: bool,
}

impl<'w> WorldRayIter<'w> {
    /// Walk voxels intersected by origin + t*dir for t ∈ [0, distance], world-bounds clipped.
    pub fn new(world: &'w World, origin: Vec3, direction: Vec3, distance: f32) -> Option<Self> {
        if !direction.is_finite() || direction == Vec3::ZERO || distance <= 0.0 {
            return None;
        }

        let grid = &world.grid;
        let block_size = grid.block_size;
        let block_extent = grid.block_extent;

        let world_aabb = AABB::new(Vec3::ZERO, Vec3::splat(grid.world_size_units));
        let (mut t0, mut t1) = ray_box_slab(origin, direction, world_aabb.min, world_aabb.max);

        if !t0.is_finite() || !t1.is_finite() {
            return None;
        }

        t0 = t0.max(0.0);
        t1 = t1.min(distance);

        if t0 > t1 {
            return None;
        }

        let eps = 1e-4;
        let mut t = (t0 + eps).min(t1);
        let p = origin + direction * t;

        let (ix, iy, iz) = grid.world_pos_to_world_ixyz(p)?; // returns (i32,i32,i32) in 0..Wx/Hx/Dx

        let voxel_indices = IVec3::default();
        let step = IVec3::default();
        let t_remaining = Vec3::default();
        let t_delta = Vec3::default();

        // Per-axis DDA setup in world coords
        let (sx, tdx, tx) = dda_axis_setup(p.x, direction.x, block_size, block_extent);
        let (sy, tdy, ty) = dda_axis_setup(p.y, direction.y, block_size, block_extent);
        let (sz, tdz, tz) = dda_axis_setup(p.z, direction.z, block_size, block_extent);

        Some(Self {
            world,
            origin,
            direction,
            t,
            max_t: t1,
            voxel_indices,
            step,
            t_delta,
            t_remaining,
            block_size,
            block_extent,
            world_aabb,
            done: false,
        })
    }
}

impl<'w> Iterator for WorldRayIter<'w> {
    type Item = VoxelSample;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let grid = &self.world.grid;

        loop {
            // Stop when we exceed the permitted segment
            if !self.tx.is_finite() && !self.ty.is_finite() && !self.tz.is_finite() {
                self.done = true;
                return None;
            }

            // Choose next boundary crossing (X→Y→Z tiebreak)
            let (axis, step, tnext) =
                min3_axis(self.tx, self.ty, self.tz, self.sx, self.sy, self.sz);
            if !tnext.is_finite() || tnext > self.max_t {
                self.done = true;
                return None;
            }

            self.t = tnext;

            // Advance accumulators and world voxel index
            match axis {
                0 => {
                    self.tx += self.tdx;
                    self.ix += self.sx;
                }
                1 => {
                    self.ty += self.tdy;
                    self.iy += self.sy;
                }
                _ => {
                    self.tz += self.tdz;
                    self.iz += self.sz;
                }
            }

            // Bounds check in WORLD voxel space
            if !grid.world_ixyz_in_bounds(self.ix, self.iy, self.iz) {
                self.done = true;
                return None;
            }

            // Face we just entered (opposite of step)
            let enter_face = face_from_axis_step(
                axis,
                match axis {
                    0 => self.sx,
                    1 => self.sy,
                    _ => self.sz,
                },
            );

            // Derive chunk + chunk-local from WORLD voxel coord (no branching here)
            let cid = grid.chunk_id_from_world_ixyz(self.ix, self.iy, self.iz);
            let (lx, ly, lz) = grid.chunk_local_from_world_ixyz(self.ix, self.iy, self.iz);
            let n = grid.chunk_size_blocks as usize;
            let local_id = (lx as usize) + (ly as usize) * n + (lz as usize) * n * n;

            let pos = self.origin + self.dir * self.t;

            return Some(VoxelSample {
                chunk_id: cid,
                ix: lx,
                iy: ly,
                iz: lz, // expose chunk-local here (aligns with the rest of your code)
                local_id,
                enter_face,
                t: self.t,
                pos,
            });
        }
    }
}

/* ================= helpers (private) ================= */

#[inline]
fn ray_box_slab(origin: Vec3, dir: Vec3, bmin: Vec3, bmax: Vec3) -> (f32, f32) {
    // (standard slab test; same as we discussed earlier)
    let (tx0, tx1) = axis_slab(origin.x, dir.x, bmin.x, bmax.x)?;
    let (ty0, ty1) = axis_slab(origin.y, dir.y, bmin.y, bmax.y)?;
    let (tz0, tz1) = axis_slab(origin.z, dir.z, bmin.z, bmax.z)?;

    let t_enter = tx0.max(ty0).max(tz0);
    let t_exit = tx1.min(ty1).min(tz1);

    if t_enter > t_exit {
        return (f32::NAN, f32::NAN);
    }

    (t_enter, t_exit)
}
#[inline]
fn axis_slab(o: f32, d: f32, mn: f32, mx: f32) -> Option<(f32, f32)> {
    if d == 0.0 {
        return if o < mn || o > mx {
            None
        } else {
            Some((f32::NEG_INFINITY, f32::INFINITY))
        };
    }

    let inv = 1.0 / d;
    let mut t0 = (mn - o) * inv;
    let mut t1 = (mx - o) * inv;

    if t0 > t1 {
        core::mem::swap(&mut t0, &mut t1);
    }

    Some((t0, t1))
}

#[inline]
fn dda_axis_setup(pos: f32, dir: f32, block_size: f32, r: f32) -> (i32, f32, f32) {
    use core::f32::INFINITY;

    let step = if dir > 0.0 {
        1
    } else if dir < 0.0 {
        -1
    } else {
        return (0, INFINITY, INFINITY);
    };

    // first plane ahead in step direction (planes at k±r)
    let next_face = if step > 0 {
        (pos - r).ceil() + r
    } else {
        (pos + r).floor() - r
    };

    let t_max = (next_face - pos) / dir;
    let t_delta = block_size / dir.abs();

    (step, t_delta, t_max)
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
